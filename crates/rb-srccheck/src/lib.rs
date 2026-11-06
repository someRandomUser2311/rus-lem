//! Source checks on the learner's code, done on parsed Rust (comments never count).

pub mod facts;

use proc_macro2::{TokenStream, TokenTree};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, Lit, Token};

/// One macro call in the source, in source order.
#[derive(Debug, Clone)]
pub struct MacroUse {
    /// Last path segment: `println`, `print`, `format`.
    pub name: String,
    /// The first argument when it's a string literal (decoded).
    pub fmt: Option<String>,
    /// Every string literal argument (decoded), including the format string.
    pub strings: Vec<String>,
    /// Integer/float literal arguments, as written.
    pub numbers: Vec<String>,
    /// Every identifier in the arguments.
    pub idents: Vec<String>,
    pub line: usize,
}

/// A function or method call.
#[derive(Debug, Clone)]
pub struct Call {
    /// Method name (`read_line`) or the called path (`String::new`, `io::stdin`).
    pub name: String,
    pub method: bool,
    /// For methods: the method names of the receiver chain, nearest first
    /// (`a().b().c()` → for `c`: `["b"]`; `a` is a path call, not a method).
    pub receivers: Vec<String>,
    /// The first argument when it's a string literal.
    pub str_arg: Option<String>,
    pub in_loop: bool,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub name: String,
    /// Normalised tokens of the whole statement, e.g. `let name = "Ferris" ;`
    pub tokens: String,
    /// Normalised type annotation and initialiser (outer parentheses removed).
    pub ty: Option<String>,
    pub init: Option<String>,
    pub in_loop: bool,
    pub line: usize,
}

#[derive(Debug, Default)]
pub struct SourceInfo {
    pub macros: Vec<MacroUse>,
    pub lets: Vec<LetStmt>,
    /// Names assigned with `name = …` / `name += …`.
    pub assigned: Vec<String>,
    /// Every identifier token in the file, including inside macro calls.
    pub idents: Vec<String>,
    pub calls: Vec<Call>,
    /// Method names called in `match` scrutinees (`match a.cmp(&b)` → `cmp`).
    pub match_methods: Vec<String>,
    /// Arithmetic operators used, with their lines.
    pub arithmetic: Vec<(String, usize)>,
    /// Structural facts (see [`facts`]).
    pub facts: Vec<facts::Fact>,
}

pub fn parse(src: &str) -> Result<SourceInfo, String> {
    let file = syn::parse_file(src).map_err(|e| {
        let at = e.span().start();
        format!("line {}: {e}", at.line)
    })?;
    let mut v = Collector::default();
    v.visit_file(&file);
    v.info.facts = facts::collect(&file);
    let tokens: TokenStream = src.parse().map_err(|e| format!("{e}"))?;
    collect_idents(tokens, &mut v.info.idents);
    Ok(v.info)
}

fn collect_idents(ts: TokenStream, out: &mut Vec<String>) {
    for t in ts {
        match t {
            TokenTree::Ident(i) => out.push(i.to_string()),
            TokenTree::Group(g) => collect_idents(g.stream(), out),
            _ => {}
        }
    }
}

#[derive(Default)]
struct Collector {
    info: SourceInfo,
    loop_depth: usize,
}

fn line_of(s: proc_macro2::Span) -> usize {
    s.start().line
}

/// Outer parentheses and groups removed.
fn unparen(e: &Expr) -> &Expr {
    match e {
        Expr::Paren(p) => unparen(&p.expr),
        Expr::Group(g) => unparen(&g.expr),
        _ => e,
    }
}

fn path_name(p: &syn::Path) -> String {
    p.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

fn str_lit(e: &Expr) -> Option<String> {
    match unparen(e) {
        Expr::Lit(l) => match &l.lit {
            Lit::Str(s) => Some(s.value()),
            _ => None,
        },
        _ => None,
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        let name = m
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        let line = m
            .path
            .segments
            .first()
            .map(|s| s.ident.span().start().line)
            .unwrap_or(0);
        let mut u = MacroUse {
            name,
            fmt: None,
            strings: Vec::new(),
            numbers: Vec::new(),
            idents: Vec::new(),
            line,
        };
        collect_idents(m.tokens.clone(), &mut u.idents);
        if let Ok(args) = m.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated) {
            for (i, a) in args.iter().enumerate() {
                collect_literals(a, &mut u);
                if i == 0
                    && let Expr::Lit(l) = a
                    && let Lit::Str(s) = &l.lit
                {
                    u.fmt = Some(s.value());
                }
                // Nested macro calls inside the arguments (format!("…") etc.).
                self.visit_expr(a);
            }
        }
        self.info.macros.push(u);
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        let (ident, ty) = match &l.pat {
            syn::Pat::Ident(p) => (Some(&p.ident), None),
            syn::Pat::Type(t) => match &*t.pat {
                syn::Pat::Ident(p) => (Some(&p.ident), Some(&*t.ty)),
                _ => (None, None),
            },
            _ => (None, None),
        };
        if let Some(ident) = ident {
            let init = l.init.as_ref().map(|i| unparen(&i.expr));
            self.info.lets.push(LetStmt {
                name: ident.to_string(),
                tokens: normalize(&quote::quote!(#l).to_string()),
                ty: ty.map(|t| normalize(&quote::quote!(#t).to_string())),
                init: init.map(|e| normalize(&quote::quote!(#e).to_string())),
                in_loop: self.loop_depth > 0,
                line: l.let_token.span.start().line,
            });
        }
        syn::visit::visit_local(self, l);
    }

    fn visit_expr_loop(&mut self, e: &'ast syn::ExprLoop) {
        self.loop_depth += 1;
        syn::visit::visit_expr_loop(self, e);
        self.loop_depth -= 1;
    }

    fn visit_expr_while(&mut self, e: &'ast syn::ExprWhile) {
        self.loop_depth += 1;
        syn::visit::visit_expr_while(self, e);
        self.loop_depth -= 1;
    }

    fn visit_expr_for_loop(&mut self, e: &'ast syn::ExprForLoop) {
        self.loop_depth += 1;
        syn::visit::visit_expr_for_loop(self, e);
        self.loop_depth -= 1;
    }

    fn visit_expr_method_call(&mut self, c: &'ast syn::ExprMethodCall) {
        let mut receivers = Vec::new();
        let mut r = unparen(&c.receiver);
        while let Expr::MethodCall(m) = r {
            receivers.push(m.method.to_string());
            r = unparen(&m.receiver);
        }
        self.info.calls.push(Call {
            name: c.method.to_string(),
            method: true,
            receivers,
            str_arg: c.args.first().and_then(str_lit),
            in_loop: self.loop_depth > 0,
            line: line_of(c.method.span()),
        });
        syn::visit::visit_expr_method_call(self, c);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let Expr::Path(p) = unparen(&c.func) {
            self.info.calls.push(Call {
                name: path_name(&p.path),
                method: false,
                receivers: Vec::new(),
                str_arg: c.args.first().and_then(str_lit),
                in_loop: self.loop_depth > 0,
                line: line_of(p.span()),
            });
        }
        syn::visit::visit_expr_call(self, c);
    }

    fn visit_expr_match(&mut self, m: &'ast syn::ExprMatch) {
        if let Expr::MethodCall(c) = unparen(&m.expr) {
            self.info.match_methods.push(c.method.to_string());
        }
        syn::visit::visit_expr_match(self, m);
    }

    fn visit_expr_unary(&mut self, u: &'ast syn::ExprUnary) {
        if let syn::UnOp::Neg(t) = u.op {
            self.info.arithmetic.push(("-".into(), line_of(t.span)));
        }
        syn::visit::visit_expr_unary(self, u);
    }

    fn visit_expr_assign(&mut self, a: &'ast syn::ExprAssign) {
        if let Expr::Path(p) = &*a.left
            && let Some(id) = p.path.get_ident()
        {
            self.info.assigned.push(id.to_string());
        }
        syn::visit::visit_expr_assign(self, a);
    }

    fn visit_expr_binary(&mut self, b: &'ast syn::ExprBinary) {
        use syn::BinOp::*;
        if matches!(
            b.op,
            Add(_)
                | Sub(_)
                | Mul(_)
                | Div(_)
                | Rem(_)
                | AddAssign(_)
                | SubAssign(_)
                | MulAssign(_)
                | DivAssign(_)
                | RemAssign(_)
        ) {
            let op = quote::ToTokens::to_token_stream(&b.op).to_string();
            self.info.arithmetic.push((op, line_of(b.op.span())));
        }
        if matches!(
            b.op,
            AddAssign(_) | SubAssign(_) | MulAssign(_) | DivAssign(_) | RemAssign(_)
        ) && let Expr::Path(p) = &*b.left
            && let Some(id) = p.path.get_ident()
        {
            self.info.assigned.push(id.to_string());
        }
        syn::visit::visit_expr_binary(self, b);
    }
}

fn collect_literals(e: &Expr, u: &mut MacroUse) {
    match e {
        Expr::Lit(l) => match &l.lit {
            Lit::Str(s) => u.strings.push(s.value()),
            Lit::Int(i) => u.numbers.push(i.to_string()),
            Lit::Float(f) => u.numbers.push(f.to_string()),
            _ => {}
        },
        Expr::Reference(r) => collect_literals(&r.expr, u),
        Expr::Paren(p) => collect_literals(&p.expr, u),
        Expr::Group(g) => collect_literals(&g.expr, u),
        _ => {}
    }
}

/// Token text with whitespace normalised, so `let x=1;` equals `let x = 1 ;`.
/// A block `{ setup; tail }` with at least one statement before its final expression, split
/// into the setup statements and the tail: `{ let s = f(); g(&s) }` → (`let s = f();`, `g(&s)`).
pub fn split_tail(code: &str) -> Option<(String, String)> {
    let block: syn::Block = syn::parse_str(code.trim()).ok()?;
    let (last, setup) = block.stmts.split_last()?;
    let syn::Stmt::Expr(tail, None) = last else {
        return None;
    };
    if setup.is_empty() {
        return None;
    }
    let setup: String = setup
        .iter()
        .map(|s| quote::quote!(#s).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    Some((setup, quote::quote!(#tail).to_string()))
}

pub fn normalize(s: &str) -> String {
    match s.parse::<TokenStream>() {
        Ok(ts) => ts
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
        Err(_) => s.split_whitespace().collect::<Vec<_>>().join(" "),
    }
}

// ------------------------------------------------------------- checks

/// Outcome of a source rule: `Ok(())` or a message for the learner.
pub type Check = Result<(), String>;

impl SourceInfo {
    pub fn count(&self, name: &str) -> usize {
        self.macros.iter().filter(|m| m.name == name).count()
    }

    pub fn macro_count(&self, name: &str, eq: Option<u32>, ge: Option<u32>) -> Check {
        let n = self.count(name) as u32;
        if let Some(eq) = eq
            && n != eq
        {
            return Err(format!(
                "Expected exactly {eq} `{name}!` call{}, found {n}.",
                plural(eq)
            ));
        }
        if let Some(ge) = ge
            && n < ge
        {
            return Err(format!(
                "Expected at least {ge} `{name}!` call{}, found {n}.",
                plural(ge)
            ));
        }
        Ok(())
    }

    /// The call `first!(first_fmt)` occurs (exactly once if asked) and the very next
    /// macro call is `then!(then_fmt)`.
    pub fn macro_sequence(
        &self,
        first: (&str, &str),
        then: (&str, &str),
        exactly_once: bool,
    ) -> Check {
        let is = |m: &MacroUse, (n, f): (&str, &str)| {
            m.name == n && m.fmt.as_deref() == Some(f) && m.strings.len() == 1
        };
        if exactly_once {
            let n = self.count(first.0);
            if n != 1 {
                return Err(format!(
                    "Expected exactly one `{}!` call, found {n}.",
                    first.0
                ));
            }
        }
        let Some(i) = self.macros.iter().position(|m| is(m, first)) else {
            return Err(format!("Expected the call `{}!({:?})`.", first.0, first.1));
        };
        match self.macros.get(i + 1) {
            Some(m) if is(m, then) => Ok(()),
            _ => Err(format!(
                "`{}!({:?})` must be followed immediately by `{}!({:?})`.",
                first.0, first.1, then.0, then.1
            )),
        }
    }

    /// No string literal passed to `macros` contains a forbidden substring (or a digit,
    /// or a number literal when `digits`).
    pub fn fmt_literal_forbid(
        &self,
        macros: &[String],
        substrings: &[String],
        digits: bool,
    ) -> Check {
        for m in self.macros.iter().filter(|m| macros.contains(&m.name)) {
            for s in &m.strings {
                for bad in substrings {
                    if s.contains(bad.as_str()) {
                        return Err(format!(
                            "Line {}: the text {:?} is typed into `{}!`. It must come from a placeholder.",
                            m.line, bad, m.name
                        ));
                    }
                }
                if digits && let Some(d) = s.chars().find(|c| c.is_ascii_digit()) {
                    return Err(format!(
                        "Line {}: the digit `{d}` is typed into the text of `{}!`. Numbers must come from placeholders.",
                        m.line, m.name
                    ));
                }
            }
            if digits && let Some(n) = m.numbers.first() {
                return Err(format!(
                    "Line {}: the number `{n}` is passed to `{}!` directly. Use the supplied names instead.",
                    m.line, m.name
                ));
            }
        }
        Ok(())
    }

    pub fn fmt_inline_placeholders(&self, macros: &[String], names: &[String]) -> Check {
        for n in names {
            let inline = format!("{{{n}}}");
            let used = self
                .macros
                .iter()
                .filter(|m| macros.contains(&m.name))
                .filter_map(|m| m.fmt.as_deref())
                .any(|f| has_placeholder(f, &inline));
            if !used {
                return Err(format!(
                    "Use the inline placeholder `{inline}` inside the text to print `{n}`."
                ));
            }
        }
        Ok(())
    }

    /// Each required `let` statement appears once, unchanged, in this order; the names
    /// are never declared again or reassigned.
    pub fn lets_unchanged(&self, required: &[String]) -> Check {
        let mut last_line = 0;
        for want in required {
            let want_n = normalize(want);
            let name = want
                .trim_start_matches("let")
                .trim()
                .trim_start_matches("mut ")
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
                .unwrap_or("")
                .to_string();
            let same_name: Vec<&LetStmt> = self.lets.iter().filter(|l| l.name == name).collect();
            match same_name.as_slice() {
                [] => {
                    return Err(format!(
                        "The line `{want}` is missing. Keep the supplied `let` lines as they are."
                    ));
                }
                [one] if one.tokens == want_n => {
                    if one.line < last_line {
                        return Err(format!(
                            "The supplied `let` lines must stay in their original order (`{want}`)."
                        ));
                    }
                    last_line = one.line;
                }
                [one] => {
                    return Err(format!(
                        "Line {}: `{want}` was changed. Keep the supplied `let` lines as they are.",
                        one.line
                    ));
                }
                _ => {
                    return Err(format!(
                        "`{name}` is declared more than once. Keep the supplied value; don't shadow it."
                    ));
                }
            }
            if self.assigned.contains(&name) {
                return Err(format!(
                    "`{name}` is given a new value. Keep the supplied value."
                ));
            }
        }
        Ok(())
    }

    pub fn banned_keyword(&self, word: &str) -> Check {
        if self.idents.iter().any(|i| i == word) {
            Err(format!("`{word}` isn't allowed in this problem."))
        } else {
            Ok(())
        }
    }

    pub fn required_keyword(&self, word: &str) -> Check {
        if self.idents.iter().any(|i| i == word) {
            Ok(())
        } else {
            Err(format!("Use `{word}` in this problem."))
        }
    }

    fn calls_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Call> + 'a {
        self.calls.iter().filter(move |c| call_is(c, name))
    }

    pub fn call_count(&self, name: &str, eq: Option<u32>, ge: Option<u32>) -> Check {
        let n = self.calls_named(name).count() as u32;
        let shown = call_shown(name);
        if let Some(eq) = eq
            && n != eq
        {
            return Err(format!(
                "Expected exactly {eq} call{} to `{shown}`, found {n}.",
                plural(eq)
            ));
        }
        if let Some(ge) = ge
            && n < ge
        {
            return Err(if n == 0 {
                format!("Your code never calls `{shown}`.")
            } else {
                format!(
                    "Expected at least {ge} call{} to `{shown}`, found {n}.",
                    plural(ge)
                )
            });
        }
        Ok(())
    }

    /// `calls` in source order: `["read_line", "expect"]` is `.read_line(…).expect(…)`.
    pub fn call_chain(&self, calls: &[String], arg: Option<&str>) -> Check {
        let Some((last, before)) = calls.split_last() else {
            return Ok(());
        };
        let found = self.calls.iter().any(|c| {
            c.method
                && &c.name == last
                && before.iter().rev().zip(&c.receivers).all(|(a, b)| a == b)
                && c.receivers.len() >= before.len()
                && arg.is_none_or(|a| c.str_arg.as_deref() == Some(a))
        });
        if found {
            return Ok(());
        }
        let shape = calls
            .iter()
            .enumerate()
            .map(|(i, c)| match (i + 1 == calls.len(), arg) {
                (true, Some(a)) => format!(".{c}({a:?})"),
                _ => format!(".{c}(…)"),
            })
            .collect::<String>();
        Err(format!("Expected a call of the form `{shape}`."))
    }

    /// See [`rb_content::Assertion::LetInit`].
    pub fn let_init(
        &self,
        name: &str,
        ty: Option<&str>,
        exprs: &[String],
        in_loop: Option<bool>,
        after: Option<&str>,
        shadows: bool,
    ) -> Check {
        let want_ty = ty.map(normalize);
        let forms: Vec<String> = exprs.iter().map(|e| normalize(e)).collect();
        let matches_form = |l: &LetStmt| l.init.as_ref().is_some_and(|i| forms.contains(i));
        let all: Vec<&LetStmt> = self.lets.iter().filter(|l| l.name == name).collect();
        let shown = format!(
            "let {name}{} = {};",
            ty.map(|t| format!(": {t}")).unwrap_or_default(),
            exprs.first().map(String::as_str).unwrap_or("…")
        );
        let hits: Vec<&LetStmt> = all.iter().copied().filter(|l| matches_form(l)).collect();
        let l = match hits.as_slice() {
            [] if all.is_empty() => return Err(format!("Expected the line `{shown}`.")),
            [] => {
                return Err(format!(
                    "Line {}: `{name}` must be set with `{shown}`.",
                    all[0].line
                ));
            }
            [one] => *one,
            _ => {
                return Err(format!(
                    "`{shown}` should appear only once; found it {} times.",
                    hits.len()
                ));
            }
        };
        if !shadows && all.len() > 1 {
            return Err(format!(
                "`{name}` is declared {} times; declare it once, with `{shown}`.",
                all.len()
            ));
        }
        if let Some(t) = &want_ty
            && l.ty.as_ref() != Some(t)
        {
            return Err(format!(
                "Line {}: give `{name}` the type `{t}`, as in `{shown}`.",
                l.line
            ));
        }
        match in_loop {
            Some(true) if !l.in_loop => {
                return Err(format!(
                    "Line {}: `{shown}` belongs inside the loop, so each round gets a new value.",
                    l.line
                ));
            }
            Some(false) if l.in_loop => {
                return Err(format!(
                    "Line {}: `{shown}` belongs before the loop, so it's set only once.",
                    l.line
                ));
            }
            _ => {}
        }
        if let Some(a) = after {
            let first = self.lets.iter().find(|x| x.name == a);
            if first.is_none_or(|f| f.line >= l.line) {
                return Err(format!(
                    "Line {}: set `{name}` after `{a}`, with its own call.",
                    l.line
                ));
            }
        }
        Ok(())
    }

    pub fn match_on(&self, method: &str) -> Check {
        if self.match_methods.iter().any(|m| m == method) {
            Ok(())
        } else {
            Err(format!(
                "Decide with a `match` on the result of `.{method}(…)`."
            ))
        }
    }

    pub fn banned_method(&self, name: &str) -> Check {
        match self.calls.iter().find(|c| c.method && c.name == name) {
            Some(c) => Err(format!(
                "Line {}: `.{name}(…)` isn't allowed in this problem.",
                c.line
            )),
            None => Ok(()),
        }
    }

    pub fn banned_arithmetic(&self) -> Check {
        match self.arithmetic.first() {
            Some((op, line)) => Err(format!(
                "Line {line}: arithmetic (`{op}`) isn't allowed in this problem; decide with `match` instead."
            )),
            None => Ok(()),
        }
    }

    pub fn macro_mention_forbid(&self, macros: &[String], name: &str) -> Check {
        let placeholder = format!("{{{name}");
        for m in self.macros.iter().filter(|m| macros.contains(&m.name)) {
            let in_fmt = m
                .fmt
                .as_deref()
                .is_some_and(|f| has_placeholder(f, &placeholder));
            if in_fmt || m.idents.iter().any(|i| i == name) {
                return Err(format!(
                    "Line {}: `{}!` prints `{name}`. Never show it to the player.",
                    m.line, m.name
                ));
            }
        }
        Ok(())
    }

    pub fn call_in_loop(&self, name: &str) -> Check {
        if self.calls_named(name).any(|c| c.in_loop) {
            Ok(())
        } else {
            Err(format!(
                "Call `{}` inside the loop, so every round starts fresh.",
                call_shown(name)
            ))
        }
    }

    /// How often `what` appears; `a|b` counts either alternative.
    fn fact_count(&self, what: &str, within: Option<&str>, inside: Option<&str>) -> usize {
        if what.contains('|') {
            return what
                .split('|')
                .map(|w| self.fact_count(w, within, inside))
                .sum();
        }
        let within = within.map(|f| format!("fn:{f}"));
        self.facts
            .iter()
            .filter(|f| {
                f.what == what
                    && within.as_ref().is_none_or(|w| f.ctx.contains(w))
                    && inside.is_none_or(|i| f.ctx.iter().any(|c| c == i))
            })
            .count()
    }

    fn has_fact(&self, what: &str, within: Option<&str>, inside: Option<&str>) -> bool {
        self.fact_count(what, within, inside) > 0
    }

    /// Every construct in `what` appears, in function `within` and inside `inside`. A
    /// construct listed n times must appear at least n times.
    pub fn uses(
        &self,
        what: &[String],
        within: Option<&str>,
        inside: Option<&str>,
        message: Option<&str>,
    ) -> Check {
        let mut missing: Vec<&String> = what
            .iter()
            .filter(|w| {
                let needed = what.iter().filter(|x| x == w).count();
                self.fact_count(w, within, inside) < needed
            })
            .collect();
        missing.dedup();
        if missing.is_empty() {
            return Ok(());
        }
        if let Some(m) = message {
            return Err(m.to_string());
        }
        let mut msg = format!(
            "Your code needs {}",
            missing
                .iter()
                .map(|w| facts_shown(w))
                .collect::<Vec<_>>()
                .join(" and ")
        );
        if let Some(i) = inside {
            msg.push_str(&format!(" inside {}", facts_shown(i)));
        }
        if let Some(f) = within {
            msg.push_str(&format!(" in `{f}`"));
        }
        msg.push('.');
        Err(msg)
    }

    /// None of the constructs in `what` appear (in function `within` when given).
    /// `what` appears exactly `eq` times (in function `within`, inside `inside`).
    pub fn fact_exactly(
        &self,
        what: &str,
        eq: usize,
        within: Option<&str>,
        inside: Option<&str>,
        message: Option<&str>,
    ) -> Check {
        let n = self.fact_count(what, within, inside);
        if n == eq {
            return Ok(());
        }
        if let Some(m) = message {
            return Err(m.to_string());
        }
        Err(format!(
            "Your code should have {} exactly {eq} time{}{}, but it has it {n} time{}.",
            facts_shown(what),
            if eq == 1 { "" } else { "s" },
            within.map(|f| format!(" in `{f}`")).unwrap_or_default(),
            if n == 1 { "" } else { "s" },
        ))
    }

    pub fn forbids(&self, what: &[String], within: Option<&str>, message: Option<&str>) -> Check {
        match what.iter().find(|w| self.has_fact(w, within, None)) {
            None => Ok(()),
            Some(_) if message.is_some() => Err(message.unwrap_or_default().to_string()),
            Some(w) => Err(format!(
                "Your code uses {}{}, which this problem doesn't allow.",
                facts_shown(w),
                within.map(|f| format!(" in `{f}`")).unwrap_or_default()
            )),
        }
    }
}

/// A fact or context in words, for messages.
fn facts_shown(what: &str) -> String {
    if what.contains('|') {
        return what
            .split('|')
            .map(facts_shown)
            .collect::<Vec<_>>()
            .join(" or ");
    }
    let (kind, arg) = what.split_once(':').unwrap_or((what, ""));
    match kind {
        "while" => "a `while` loop".into(),
        "loop" => "a `loop`".into(),
        "for" => "a `for` loop".into(),
        "for_range" => "a `for` loop over a range".into(),
        "for_rev" => "a `for` loop over a reversed range (`.rev()`)".into(),
        "for_ref" => "a `for &value in …` loop that copies each element".into(),
        "reborrow" => "a shared reborrow (`&*text`)".into(),
        "reborrow_mut" => "a mutable reborrow (`&mut *text`)".into(),
        "labeled_loop" => "a labeled loop (`'outer: loop`)".into(),
        "labeled_break" => "a labeled `break`".into(),
        "break_value" => "`break` with a value".into(),
        "op" => format!("the `{arg}` operator"),
        "cast" => format!("an `as {arg}` cast"),
        "call" => format!("a call to `{arg}`"),
        "recursion" => "recursion (a function calling itself)".into(),
        "let" => format!("a `let {arg}` binding"),
        "let_ty" => format!("a `let` with the type annotation `: {arg}`"),
        "let_tuple" => "a `let` that destructures a tuple".into(),
        "let_mut" => format!("a `let mut {arg}` binding"),
        "shadow" => format!("a second `let {arg}` that shadows the first"),
        "assign" => format!("an assignment to `{arg}`"),
        "const" => format!("the constant `{arg}`"),
        "const_ty" => format!("a constant of type `{arg}`"),
        "ident" => format!("`{arg}`"),
        "if" => "an `if`".into(),
        "else_if" => "an `else if`".into(),
        "if_expr" => "an `if … else` expression that produces a value".into(),
        "block_expr" => "a block expression `{ let …; value }` that produces a value".into(),
        "index" => "array indexing (`a[i]`)".into(),
        "float" => "floating-point values (`f64`, `1.5`)".into(),
        "tuple" => format!("a tuple with {arg} fields"),
        "tail" => format!("`{arg}` as the function's final expression"),
        "fn" => format!("the function `{arg}`"),
        "type" => format!("the type `{arg}`"),
        "macro" => format!("the `{arg}!` macro"),
        "item" => match arg {
            "fn" => "a function".into(),
            "const" => "a constant".into(),
            "use" => "a `use` declaration".into(),
            _ => format!("a `{arg}` definition"),
        },
        "lifetime" => "an explicit lifetime annotation (`'a`)".into(),
        "unsafe" => "`unsafe` code".into(),
        "struct" => format!("the struct `{arg}`"),
        "tuple_struct" => format!("`{arg}` as a tuple struct"),
        "unit_struct" => format!("`{arg}` as a unit struct"),
        "derive" => format!("`#[derive({arg})]`"),
        "field" => match arg.split_once(':') {
            Some((name, ty)) => format!("the field `{name}: {ty}`"),
            None => format!("the field `{arg}`"),
        },
        "impl" => format!("an `impl {arg}` block"),
        "multi_impl" => format!("a second `impl {arg}` block"),
        "trait_impl" if arg.is_empty() => "a trait implementation (`impl Trait for Type`)".into(),
        "trait_impl" => format!("an `impl {arg} for …` block"),
        "recv" => match arg {
            "ref" => "a `&self` receiver".into(),
            "mut" => "a `&mut self` receiver".into(),
            "own" => "a `self` receiver that takes ownership".into(),
            _ => "no `self` receiver (an associated function)".into(),
        },
        "ret" => format!("the return type `{arg}`"),
        "struct_lit" => format!("a `{arg} {{ … }}` struct expression"),
        "enum" => format!("the enum `{arg}`"),
        "variant" => format!("the variant `{arg}`"),
        "variant_shape" => format!("the variant `{arg}`"),
        "match" => "a `match`".into(),
        "if_let" => "an `if let`".into(),
        "while_let" => "a `while let` loop".into(),
        "let_else" => "a `let … else` binding".into(),
        "pat" => format!("a `{arg}` pattern"),
        "pat_or" => "an or-pattern (`a | b`)".into(),
        "pat_range" => "a range pattern".into(),
        "pat_range_incl" => "an inclusive range pattern (`a..=b`)".into(),
        "pat_wild" => "a `_` catch-all arm".into(),
        "pat_bind" => "a catch-all arm that binds a name".into(),
        "arm_guard" => "a match guard (`if` on an arm)".into(),
        "try" => "the `?` operator".into(),
        "shorthand" => format!("the field init shorthand `{arg}`"),
        "update" => "struct update syntax (`..base`)".into(),
        "mod" => format!("the inline module `mod {arg} {{ … }}`"),
        "mod_file" => format!("the declaration `mod {arg};` (its body in its own file)"),
        "mod_decl" => format!("a `mod {arg}` declaration"),
        "pub" => format!("`pub` on `{arg}`"),
        "pub_restricted" => format!("`pub(crate)` or `pub(super)` on `{arg}`"),
        "pub_field" => format!("`pub` on the field `{arg}`"),
        "use" => format!("`use {arg};`"),
        "pub_use" => format!("`pub use {arg};`"),
        "use_glob" => "a glob import (`use …::*`)".into(),
        "use_group" => "a nested `use` with braces (`use a::{b, c}`)".into(),
        "call_path" => format!("a call to `{arg}`"),
        "path" => format!("the path `{arg}`"),
        "lit" => format!("the literal `{arg}`"),
        "attr" => format!("a `#[{arg}]` attribute"),
        "closure" => "a closure (`|x| …`)".into(),
        "closure_move" => "a `move` closure".into(),
        "range" => "a range (`a..b` or `a..=b`)".into(),
        "doc_crate" => "a crate-level `//!` doc comment".into(),
        "doc" => format!("`///` documentation on `{arg}`"),
        "doc_heading" => format!("a `# {arg}` section in a doc comment"),
        "doc_code" => "a code example (a ``` fence) in a doc comment".into(),
        _ => format!("`{what}`"),
    }
}

/// Does call `c` match `name`: a method name, or a path matched on whole trailing
/// segments (`String::new` matches `String::new` and `std::string::String::new`).
fn call_is(c: &Call, name: &str) -> bool {
    if c.method {
        return !name.contains("::") && c.name == name;
    }
    c.name == name || c.name.ends_with(&format!("::{name}"))
}

fn call_shown(name: &str) -> String {
    if name.contains("::") {
        format!("{name}()")
    } else {
        format!(".{name}(…)")
    }
}

/// Replaces the initialiser of each named `let` with a read of the fixture value from
/// the environment (`RB_FIXTURE_<name>`), so one build serves every fixture value.
/// Each binding must be declared exactly once.
pub fn instrument(src: &str, bindings: &[String]) -> Result<String, String> {
    let file = syn::parse_file(src).map_err(|e| format!("line {}: {e}", e.span().start().line))?;
    struct Finder<'a> {
        names: &'a [String],
        found: Vec<(String, proc_macro2::Span)>,
    }
    impl<'ast> Visit<'ast> for Finder<'_> {
        fn visit_local(&mut self, l: &'ast syn::Local) {
            let ident = match &l.pat {
                syn::Pat::Ident(p) => Some(&p.ident),
                syn::Pat::Type(t) => match &*t.pat {
                    syn::Pat::Ident(p) => Some(&p.ident),
                    _ => None,
                },
                _ => None,
            };
            if let (Some(id), Some(init)) = (ident, &l.init)
                && self.names.iter().any(|n| id == n)
            {
                self.found.push((id.to_string(), init.expr.span()));
            }
            syn::visit::visit_local(self, l);
        }
    }
    let mut f = Finder {
        names: bindings,
        found: Vec::new(),
    };
    f.visit_file(&file);
    for b in bindings {
        match f.found.iter().filter(|(n, _)| n == b).count() {
            0 => {
                return Err(format!(
                    "Keep the line that sets `{b}` with `let {b} = …;`."
                ));
            }
            1 => {}
            n => {
                return Err(format!(
                    "`{b}` is declared {n} times; declare it once so the tests can find it."
                ));
            }
        }
    }
    // Line starts, then (line, char column) → byte offset.
    let starts: Vec<usize> = std::iter::once(0)
        .chain(src.match_indices('\n').map(|(i, _)| i + 1))
        .collect();
    let offset = |lc: proc_macro2::LineColumn| -> usize {
        let start = starts.get(lc.line - 1).copied().unwrap_or(src.len());
        src[start..]
            .char_indices()
            .nth(lc.column)
            .map(|(i, _)| start + i)
            .unwrap_or(src.len())
    };
    let mut edits: Vec<(usize, usize, String)> = f
        .found
        .iter()
        .map(|(name, span)| {
            (
                offset(span.start()),
                offset(span.end()),
                format!(
                    "{{ let __rb_fixture: ::std::string::String = ::std::env::var(\"RB_FIXTURE_{name}\").expect(\"fixture value\"); __rb_fixture.trim().parse().expect(\"fixture value\") }}"
                ),
            )
        })
        .collect();
    edits.sort_by_key(|e| std::cmp::Reverse(e.0));
    let mut out = src.to_string();
    for (a, b, text) in edits {
        out.replace_range(a..b, &text);
    }
    Ok(out)
}

/// Does the format string contain the placeholder `{name}` (not the escaped text `{{name}}`)?
fn has_placeholder(fmt: &str, placeholder: &str) -> bool {
    let b = fmt.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'{' {
            if b.get(i + 1) == Some(&b'{') {
                i += 2;
                continue;
            }
            if fmt[i..].starts_with(placeholder) {
                return true;
            }
        } else if b[i] == b'}' && b.get(i + 1) == Some(&b'}') {
            i += 2;
            continue;
        }
        i += 1;
    }
    false
}

fn plural(n: u32) -> &'static str {
    if n == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = r#"
fn main() {
    let dish = "Pancakes";
    let price = 450;
    // println!("Pancakes 450") in a comment doesn't count
    print!("Rating: ");
    println!("{price} stars");
    println!("Order code: {{{dish}-{price}}}");
}
"#;

    #[test]
    fn macros_and_literals() {
        let s = parse(SRC).unwrap();
        assert_eq!(s.count("println"), 2);
        assert_eq!(s.count("print"), 1);
        assert!(
            s.macro_sequence(("print", "Rating: "), ("println", "{price} stars"), true)
                .is_ok()
        );
        assert!(
            s.fmt_literal_forbid(
                &["println".into(), "print".into()],
                &["Pancakes".into()],
                true
            )
            .is_ok()
        );
        let bad = parse("fn main() { let d = 1; println!(\"Price: 450 {d}\"); }").unwrap();
        assert!(
            bad.fmt_literal_forbid(&["println".into()], &[], true)
                .is_err()
        );
        let bad = parse("fn main() { println!(\"{}\", 450); }").unwrap();
        assert!(
            bad.fmt_literal_forbid(&["println".into()], &[], true)
                .is_err()
        );
    }

    #[test]
    fn lets() {
        let s = parse(SRC).unwrap();
        let req = vec![
            "let dish = \"Pancakes\";".to_string(),
            "let price = 450;".to_string(),
        ];
        assert!(s.lets_unchanged(&req).is_ok());
        let changed = parse("fn main() { let dish = \"Waffles\"; let price = 450; }").unwrap();
        assert!(changed.lets_unchanged(&req).is_err());
        let shadow =
            parse("fn main() { let dish = \"Pancakes\"; let price = 450; let dish = 3; }").unwrap();
        assert!(shadow.lets_unchanged(&req).is_err());
        let mutated =
            parse("fn main() { let dish = \"Pancakes\"; let mut price = 450; price = 3; }")
                .unwrap();
        assert!(mutated.lets_unchanged(&req).is_err());
    }

    #[test]
    fn inline_placeholders() {
        let s = parse("fn main() { let name = 1; println!(\"{{name}} {{{name}}}\"); }").unwrap();
        assert!(
            s.fmt_inline_placeholders(&["println".into()], &["name".into()])
                .is_ok()
        );
        let s = parse("fn main() { let name = 1; println!(\"{{name}} {}\", name); }").unwrap();
        assert!(
            s.fmt_inline_placeholders(&["println".into()], &["name".into()])
                .is_err()
        );
    }

    const GAME: &str = r#"
use rand::prelude::*;
use std::io;
fn main() {
    let secret_number: u32 = rand::rng().random_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() { Ok(n) => n, Err(_) => continue };
        match guess.cmp(&secret_number) { _ => break }
    }
    println!("é {}", 1);
}
"#;

    #[test]
    fn calls_and_lets() {
        let s = parse(GAME).unwrap();
        assert!(s.call_count("read_line", Some(1), None).is_ok());
        assert!(s.call_count("String::new", Some(1), None).is_ok());
        assert!(s.call_count("max", None, Some(1)).is_err());
        assert!(
            s.call_chain(
                &["read_line".into(), "expect".into()],
                Some("Failed to read line")
            )
            .is_ok()
        );
        assert!(
            s.call_chain(&["read_line".into(), "expect".into()], Some("x"))
                .is_err()
        );
        assert!(s.call_in_loop("String::new").is_ok());
        assert!(s.match_on("cmp").is_ok());
        let secret = |in_loop| {
            s.let_init(
                "secret_number",
                Some("u32"),
                &["rand::rng().random_range(1..=100)".into()],
                in_loop,
                None,
                false,
            )
        };
        assert!(secret(Some(false)).is_ok());
        assert!(secret(Some(true)).is_err());
        assert!(
            s.let_init("guess", Some("u32"), &["x".into()], None, None, true)
                .is_err()
        );
        assert!(s.banned_arithmetic().is_ok());
        assert!(
            parse("fn main() { let a = 1 + 2; }")
                .unwrap()
                .banned_arithmetic()
                .is_err()
        );
        assert!(
            parse("fn main() { let a = -2; }")
                .unwrap()
                .banned_arithmetic()
                .is_err()
        );
        assert!(
            s.macro_mention_forbid(&["println".into()], "secret_number")
                .is_ok()
        );
        let leak =
            parse("fn main() { let secret_number = 1; println!(\"{secret_number:?}\"); }").unwrap();
        assert!(
            leak.macro_mention_forbid(&["println".into()], "secret_number")
                .is_err()
        );
    }

    #[test]
    fn instruments_a_binding() {
        let out = instrument(GAME, &["secret_number".into()]).unwrap();
        assert!(out.contains("let secret_number: u32 = { let __rb_fixture"));
        assert!(!out.contains("random_range(1..=100)"));
        assert!(out.contains("println!(\"é {}\", 1);"));
        assert!(instrument(GAME, &["missing".into()]).is_err());
        let twice = "fn main() { let x: u32 = 1; let x: u32 = 2; }";
        assert!(instrument(twice, &["x".into()]).is_err());
    }

    #[test]
    fn keywords() {
        let s = parse("fn main() { loop { break; } }").unwrap();
        assert!(s.banned_keyword("while").is_ok());
        assert!(s.banned_keyword("loop").is_err());
    }
}
