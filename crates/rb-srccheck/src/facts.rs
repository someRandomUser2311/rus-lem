//! Structural facts: which constructs the code uses, and what encloses each one.
//!
//! A fact is a construct name with its context, the enclosing constructs from the
//! outside in. Rules ask whether a construct appears, optionally inside a function
//! (`fn:gcd`) or inside another construct (`while`).
//!
//! | Fact | Meaning |
//! |---|---|
//! | `fn:NAME` | a function definition |
//! | `while`, `loop`, `for` | a loop |
//! | `for_range` | a `for` over a range (`a..b`, `a..=b`, also `(a..=b).rev()`) |
//! | `for_rev` | a `for` over a reversed range |
//! | `for_ref` | a `for` whose pattern copies through a reference (`for &value in values`) |
//! | `reborrow`, `reborrow_mut` | `&*x`, `&mut *x` |
//! | `labeled_loop`, `labeled_break` | a loop with a label, a `break 'label` |
//! | `break_value` | `break` with a value |
//! | `op:+` … `op:%`, `op:+=` … | arithmetic, compound assignment (also counts as `op:+`) |
//! | `cast:TYPE` | `as TYPE` |
//! | `call:NAME` | a call of a function or method (last path segment) |
//! | `recursion` | the function calls itself, directly or through others in the file |
//! | `let:NAME`, `let_ty:TYPE`, `let_tuple`, `let_mut:NAME`, `shadow:NAME` | `let` statements |
//! | `assign:NAME` | `NAME = …` or a compound assignment to it |
//! | `const:NAME`, `const_ty:TYPE` | a `const` item |
//! | `ident:NAME` | a path expression naming NAME |
//! | `if`, `else_if`, `if_expr` | `if`, `else if`, an `if … else` whose branches produce values |
//! | `block_expr` | a value-producing `{ let …; tail }` block that isn't a body |
//! | `index` | indexing `a[i]` |
//! | `float` | an `f32`/`f64` type or a float literal |
//! | `tuple:N` | a tuple expression with N fields |
//! | `tail:NAME` | a function body ending in the expression NAME |
//! | `type:NAME` | a type naming NAME (`Vec<i32>`, `Option<u8>`: `type:Vec`, `type:Option`) |
//! | `macro:NAME` | a macro call (`vec!`: `macro:vec`) |
//! | `item:KIND` | an item other than a function or constant: `struct`, `enum`, `trait`, `impl`, `type`, `static`, `mod` |
//! | `lifetime` | an explicit lifetime (`'a`, `'static`), not a loop label |
//! | `unsafe` | an `unsafe` block or function |
//! | `struct:NAME`, `tuple_struct:NAME`, `unit_struct:NAME` | a struct definition (every kind is also `struct:NAME`) |
//! | `derive:TRAIT` | a derive on the enclosing `struct:NAME` |
//! | `field:NAME:TYPE` | a named field of the enclosing `struct:NAME` |
//! | `impl:TYPE`, `multi_impl:TYPE` | an `impl` block; a second one for the same type |
//! | `trait_impl`, `trait_impl:TRAIT` | an `impl TRAIT for TYPE` block |
//! | `recv:ref`, `recv:mut`, `recv:own`, `recv:none` | a function's receiver: `&self`, `&mut self`, `self`, none |
//! | `ret:TYPE` | a function's return type (`ret:()` when it has none) |
//! | `struct_lit:NAME` | a struct expression `NAME { … }` |
//! | `shorthand:FIELD`, `update` | inside `struct_lit:NAME`: a shorthand field, a `..base` update |
//! | `enum:NAME` | an enum definition (`derive:TRAIT` inside it as for structs) |
//! | `variant:NAME`, `variant_shape:TEXT` | inside `enum:NAME`: a variant, and its exact shape (`Move{x:i32,y:i32}`) |
//! | `match`, `if_let`, `while_let`, `let_else` | a `match`, `if let`, `while let`, `let … else` |
//! | `pat:NAME` | a pattern naming a variant or other capitalised path (`Some(n)`, `Coin::Penny`: `pat:Some`, `pat:Penny`) |
//! | `pat_or`, `pat_range`, `pat_range_incl` | an or-pattern `a \| b`; a range pattern; an inclusive one (`1..=9`) |
//! | `pat_wild`, `pat_bind`, `arm_guard` | a `match` arm that is `_`, that binds everything to a name, that has an `if` guard |
//! | `try` | the `?` operator |
//! | `item:fn`, `item:const`, `item:use` | a function (also a method), a constant, a `use` declaration |
//! | `mod:NAME`, `mod_file:NAME`, `mod_decl:NAME` | an inline module `mod NAME { … }`; a body-less `mod NAME;` (its body is in a file); either |
//! | `pub:NAME`, `pub_restricted:NAME` | a `pub` item (function, struct, enum, module, constant); also a `pub(crate)`/`pub(super)` one, which is both |
//! | `pub_field:NAME` | inside `struct:NAME`: a field with a visibility modifier |
//! | `use:PATH`, `pub_use:PATH` | one name a `use` (or `pub use`) brings in, its path written out whole: `crate::a::b`, `a::B as C`, `a::*` (`a::{self}` is `a`) |
//! | `use_glob`, `use_group` | a `use` with a glob `*`; with a braced group `{…}` |
//! | `call_path:PATH`, `path:PATH` | a call through a path of two or more segments (`crate::a::f`, `super::f`); any such path expression (`super::LIMIT`) |
//! | `lit:N` | an integer literal with value N |
//! | `attr:path` | a `#[path = "…"]` attribute |
//! | `cfg_test` | an item under `#[cfg(test)]` (also a context: what's inside it) |
//! | `attr:test`, `test_fn:NAME` | a function marked `#[test]` |
//! | `stray_test` | a `#[test]` function outside every `#[cfg(test)]` item (fine in `tests/*.rs`) |
//! | `attr:should_panic`, `should_panic_expected`, `should_panic_bare` | `#[should_panic]`; one with `expected = "…"`; one without |
//! | `assert_msg`, `assert_bare` | an `assert!`, `assert_eq!` or `assert_ne!` (also `debug_` ones) with a custom failure message; one without |
//! | `attr:ignore`, `ignore_reason` | `#[ignore]`; one with a reason (`#[ignore = "slow"]`) |
//! | `closure` | a closure expression (`\|x\| …`) |
//! | `trait:NAME` | a trait definition |
//! | `required_fn:NAME`, `default_fn:NAME` | inside `trait:NAME`: a method without a body; one with a default body (also `fn:NAME`) |
//! | `bound`, `bound:TRAIT` | a trait bound, in generics, a `where` clause or an `impl TRAIT` type (`T: Clone + Display`: `bound:Clone`, `bound:Display`) |
//! | `borrow`, `deref`, `field_access:NAME` | an expression `&x` (or `&mut x`), `*x`, `x.NAME` (`x.0`: `field_access:0`) |
//! | `where` | a `where` clause |
//! | `dyn` | a trait object type (`dyn Error`) |
//! | `lifetime:NAME` | an explicit lifetime by name, alongside `lifetime` (`'static`: `lifetime:static`) |
//!
//! Contexts are `fn:NAME`, `mod:NAME`, the loop facts, `if`, `block_expr`, `let:NAME` (its
//! initialiser), `const:NAME`, `struct:NAME`, `enum:NAME`, `trait:NAME`, `impl:TYPE` (and
//! `impl_unbounded` for one whose type parameters have no bounds), `struct_lit:NAME`,
//! `match`, `if_let` (its pattern and `then` block), `while_let` and `let_else` (its
//! pattern and `else` block).

use std::collections::{BTreeMap, BTreeSet};
use syn::visit::Visit;
use syn::{Expr, Stmt};

#[derive(Debug, Clone)]
pub struct Fact {
    pub what: String,
    pub ctx: Vec<String>,
}

pub fn collect(file: &syn::File) -> Vec<Fact> {
    let mut v = Facts::default();
    v.visit_file(file);
    // Recursion: a function that reaches itself through calls to the file's functions.
    let fns: BTreeSet<&String> = v.calls.keys().collect();
    for f in &fns {
        let mut seen = BTreeSet::new();
        let mut todo: Vec<&String> = v.calls[*f].iter().filter(|c| fns.contains(c)).collect();
        while let Some(c) = todo.pop() {
            if c == *f {
                v.out.push(Fact {
                    what: "recursion".into(),
                    ctx: vec![format!("fn:{f}")],
                });
                break;
            }
            if seen.insert(c) {
                todo.extend(v.calls[c].iter().filter(|c| fns.contains(c)));
            }
        }
    }
    v.out
}

#[derive(Default)]
struct Facts {
    out: Vec<Fact>,
    ctx: Vec<String>,
    /// Per function: the plain function names it calls.
    calls: BTreeMap<String, BTreeSet<String>>,
    /// `let` names seen in the current function.
    lets: BTreeSet<String>,
    /// Types with an `impl` block so far.
    impls: BTreeSet<String>,
}

/// A path's segments joined by `::`, without generic arguments.
fn path_text(p: &syn::Path) -> String {
    let segs: Vec<String> = p.segments.iter().map(|s| s.ident.to_string()).collect();
    let lead = if p.leading_colon.is_some() { "::" } else { "" };
    format!("{lead}{}", segs.join("::"))
}

fn ty_text(t: &syn::Type) -> String {
    crate::normalize(&quote::quote!(#t).to_string()).replace(' ', "")
}

fn unparen(e: &Expr) -> &Expr {
    match e {
        Expr::Paren(p) => unparen(&p.expr),
        Expr::Group(g) => unparen(&g.expr),
        _ => e,
    }
}

fn is_range(e: &Expr) -> bool {
    match unparen(e) {
        Expr::Range(_) => true,
        Expr::MethodCall(m) => is_range(&m.receiver),
        _ => false,
    }
}

fn is_rev_range(e: &Expr) -> bool {
    match unparen(e) {
        Expr::MethodCall(m) => {
            (m.method == "rev" && is_range(&m.receiver)) || is_rev_range(&m.receiver)
        }
        _ => false,
    }
}

/// A block that ends in an expression producing a value (not `println!`, not an assignment).
fn yields_value(b: &syn::Block) -> bool {
    match b.stmts.last() {
        Some(Stmt::Expr(e, None)) => match unparen(e) {
            Expr::Assign(_) | Expr::While(_) | Expr::ForLoop(_) => false,
            Expr::Macro(m) => !m
                .mac
                .path
                .segments
                .last()
                .is_some_and(|s| s.ident.to_string().contains("print")),
            Expr::If(i) => if_yields(i),
            Expr::Block(b) => yields_value(&b.block),
            _ => true,
        },
        // `return x;` and `break x;` leave the block with a value too.
        Some(Stmt::Expr(Expr::Return(r), _)) => r.expr.is_some(),
        _ => false,
    }
}

/// A condition that is or contains a `let` (`if let …`, `if let … && …`).
fn has_let(e: &Expr) -> bool {
    match unparen(e) {
        Expr::Let(_) => true,
        Expr::Binary(b) => has_let(&b.left) || has_let(&b.right),
        _ => false,
    }
}

fn if_yields(i: &syn::ExprIf) -> bool {
    let Some((_, els)) = &i.else_branch else {
        return false;
    };
    yields_value(&i.then_branch)
        && match unparen(els) {
            Expr::Block(b) => yields_value(&b.block),
            Expr::If(i) => if_yields(i),
            _ => true,
        }
}

impl Facts {
    fn fact(&mut self, what: impl Into<String>) {
        self.out.push(Fact {
            what: what.into(),
            ctx: self.ctx.clone(),
        });
    }

    fn within<T>(&mut self, ctx: impl Into<String>, f: impl FnOnce(&mut Self) -> T) -> T {
        self.ctx.push(ctx.into());
        let r = f(self);
        self.ctx.pop();
        r
    }

    fn current_fn(&self) -> Option<String> {
        self.ctx
            .iter()
            .rev()
            .find_map(|c| c.strip_prefix("fn:").map(str::to_string))
    }

    fn function(
        &mut self,
        sig: &syn::Signature,
        block: &syn::Block,
        visit: impl FnOnce(&mut Self),
    ) {
        let name = sig.ident.to_string();
        self.fact(format!("fn:{name}"));
        self.calls.entry(name.clone()).or_default();
        let outer = std::mem::take(&mut self.lets);
        self.within(format!("fn:{name}"), |v| {
            let recv = match sig.receiver() {
                None => "none",
                Some(r) if r.reference.is_none() => "own",
                Some(r) if r.mutability.is_some() => "mut",
                Some(_) => "ref",
            };
            v.fact(format!("recv:{recv}"));
            v.fact(format!(
                "ret:{}",
                match &sig.output {
                    syn::ReturnType::Default => "()".to_string(),
                    syn::ReturnType::Type(_, t) => ty_text(t),
                }
            ));
            if let Some(Stmt::Expr(e, None)) = block.stmts.last()
                && let Expr::Path(p) = unparen(e)
                && let Some(id) = p.path.get_ident()
            {
                v.fact(format!("tail:{id}"));
            }
            visit(v);
        });
        self.lets = outer;
    }

    /// `pub:NAME` (and `pub_restricted:NAME`) for an item with a visibility modifier.
    fn visibility(&mut self, name: &str, vis: &syn::Visibility) {
        match vis {
            syn::Visibility::Public(_) => self.fact(format!("pub:{name}")),
            syn::Visibility::Restricted(_) => {
                self.fact(format!("pub:{name}"));
                self.fact(format!("pub_restricted:{name}"));
            }
            syn::Visibility::Inherited => {}
        }
    }

    /// The names a `use` tree brings in, each with its whole path.
    fn use_tree(&mut self, prefix: &str, t: &syn::UseTree, public: bool) {
        let join = |p: &str, s: &str| {
            if p.is_empty() {
                s.to_string()
            } else {
                format!("{p}::{s}")
            }
        };
        let leaf = |v: &mut Self, path: String| {
            v.fact(format!("use:{path}"));
            if public {
                v.fact(format!("pub_use:{path}"));
            }
        };
        match t {
            syn::UseTree::Path(p) => {
                self.use_tree(&join(prefix, &p.ident.to_string()), &p.tree, public)
            }
            syn::UseTree::Name(n) if n.ident == "self" => leaf(self, prefix.to_string()),
            syn::UseTree::Name(n) => leaf(self, join(prefix, &n.ident.to_string())),
            syn::UseTree::Rename(r) => {
                let from = if r.ident == "self" {
                    prefix.to_string()
                } else {
                    join(prefix, &r.ident.to_string())
                };
                leaf(self, format!("{from} as {}", r.rename));
            }
            syn::UseTree::Glob(_) => {
                self.fact("use_glob");
                leaf(self, join(prefix, "*"));
            }
            syn::UseTree::Group(g) => {
                self.fact("use_group");
                for t in &g.items {
                    self.use_tree(prefix, t, public);
                }
            }
        }
    }

    fn derives(&mut self, attrs: &[syn::Attribute]) {
        for a in attrs {
            if a.path().is_ident("derive") {
                let _ = a.parse_nested_meta(|m| {
                    if let Some(last) = m.path.segments.last() {
                        self.fact(format!("derive:{}", last.ident));
                    }
                    Ok(())
                });
            }
        }
    }

    fn pattern_lets(&mut self, pat: &syn::Pat) {
        match pat {
            syn::Pat::Ident(p) => {
                let name = p.ident.to_string();
                if self.lets.contains(&name) {
                    self.fact(format!("shadow:{name}"));
                }
                self.lets.insert(name.clone());
                self.fact(format!("let:{name}"));
                if p.mutability.is_some() {
                    self.fact(format!("let_mut:{name}"));
                }
            }
            syn::Pat::Type(t) => {
                self.fact(format!("let_ty:{}", ty_text(&t.ty)));
                self.pattern_lets(&t.pat);
            }
            syn::Pat::Tuple(t) => {
                self.fact("let_tuple");
                for p in &t.elems {
                    self.pattern_lets(p);
                }
            }
            _ => {}
        }
    }
}

/// `#[cfg(test)]`
fn is_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path().is_ident("cfg") && a.parse_args::<syn::Ident>().is_ok_and(|i| i == "test")
    })
}

impl Facts {
    /// `#[test]`, `#[should_panic]` and `#[ignore]` on a function.
    fn test_attrs(&mut self, name: &str, attrs: &[syn::Attribute]) {
        for a in attrs {
            let p = a.path();
            if p.is_ident("test") {
                self.fact("attr:test");
                self.fact(format!("test_fn:{name}"));
                if !self.ctx.iter().any(|c| c == "cfg_test") && !is_cfg_test(attrs) {
                    self.fact("stray_test");
                }
            } else if p.is_ident("should_panic") {
                self.fact("attr:should_panic");
                if matches!(a.meta, syn::Meta::List(_)) {
                    let mut expected = false;
                    let _ = a.parse_nested_meta(|m| {
                        if m.path.is_ident("expected") {
                            expected = true;
                            let _: syn::LitStr = m.value()?.parse()?;
                        }
                        Ok(())
                    });
                    if expected {
                        self.fact("should_panic_expected");
                    } else {
                        self.fact("should_panic_bare");
                    }
                } else {
                    self.fact("should_panic_bare");
                }
            } else if p.is_ident("ignore") {
                self.fact("attr:ignore");
                if matches!(a.meta, syn::Meta::NameValue(_)) {
                    self.fact("ignore_reason");
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for Facts {
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        self.visibility(&f.sig.ident.to_string(), &f.vis);
        self.test_attrs(&f.sig.ident.to_string(), &f.attrs);
        if is_cfg_test(&f.attrs) {
            self.fact("cfg_test");
            return self.within("cfg_test", |v| {
                v.function(&f.sig, &f.block, |v| syn::visit::visit_item_fn(v, f))
            });
        }
        self.function(&f.sig, &f.block, |v| syn::visit::visit_item_fn(v, f));
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        self.fact("item:fn");
        self.visibility(&f.sig.ident.to_string(), &f.vis);
        self.function(&f.sig, &f.block, |v| syn::visit::visit_impl_item_fn(v, f));
    }

    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        let name = m.ident.to_string();
        self.visibility(&name, &m.vis);
        self.fact(format!("mod_decl:{name}"));
        if m.attrs.iter().any(|a| a.path().is_ident("path")) {
            self.fact("attr:path");
        }
        let cfg_test = is_cfg_test(&m.attrs);
        if cfg_test {
            self.fact("cfg_test");
        }
        match &m.content {
            None => self.fact(format!("mod_file:{name}")),
            Some(_) if cfg_test => {
                self.fact(format!("mod:{name}"));
                self.within("cfg_test", |v| {
                    v.within(format!("mod:{name}"), |v| syn::visit::visit_item_mod(v, m))
                });
            }
            Some(_) => {
                self.fact(format!("mod:{name}"));
                self.within(format!("mod:{name}"), |v| syn::visit::visit_item_mod(v, m));
            }
        }
    }

    fn visit_item_use(&mut self, u: &'ast syn::ItemUse) {
        let public = !matches!(u.vis, syn::Visibility::Inherited);
        let prefix = if u.leading_colon.is_some() { "::" } else { "" };
        // A leading `::` stays on the first segment.
        match prefix {
            "" => self.use_tree("", &u.tree, public),
            _ => {
                let mut t = String::new();
                let tree = &u.tree;
                t.push_str(&quote::quote!(#tree).to_string().replace(' ', ""));
                self.fact(format!("use:::{t}"));
            }
        }
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        self.fact("closure");
        syn::visit::visit_expr_closure(self, c);
    }

    fn visit_lit_int(&mut self, l: &'ast syn::LitInt) {
        self.fact(format!("lit:{}", l.base10_digits()));
    }

    fn visit_item_struct(&mut self, s: &'ast syn::ItemStruct) {
        let name = s.ident.to_string();
        self.visibility(&name, &s.vis);
        self.fact(format!("struct:{name}"));
        match &s.fields {
            syn::Fields::Unnamed(_) => self.fact(format!("tuple_struct:{name}")),
            syn::Fields::Unit => self.fact(format!("unit_struct:{name}")),
            syn::Fields::Named(_) => {}
        }
        self.within(format!("struct:{name}"), |v| {
            v.derives(&s.attrs);
            for f in &s.fields {
                if let Some(id) = &f.ident {
                    v.fact(format!("field:{id}:{}", ty_text(&f.ty)));
                    if !matches!(f.vis, syn::Visibility::Inherited) {
                        v.fact(format!("pub_field:{id}"));
                    }
                }
            }
            syn::visit::visit_item_struct(v, s)
        });
    }

    fn visit_item_enum(&mut self, e: &'ast syn::ItemEnum) {
        let name = e.ident.to_string();
        self.visibility(&name, &e.vis);
        self.fact(format!("enum:{name}"));
        self.within(format!("enum:{name}"), |v| {
            v.derives(&e.attrs);
            for var in &e.variants {
                v.fact(format!("variant:{}", var.ident));
                let shape = quote::quote!(#var).to_string().replace(' ', "");
                v.fact(format!("variant_shape:{}", shape.trim_end_matches(',')));
            }
            syn::visit::visit_item_enum(v, e)
        });
    }

    fn visit_expr_match(&mut self, m: &'ast syn::ExprMatch) {
        self.fact("match");
        self.visit_expr(&m.expr);
        self.within("match", |v| {
            for arm in &m.arms {
                match &arm.pat {
                    syn::Pat::Wild(_) => v.fact("pat_wild"),
                    syn::Pat::Ident(p)
                        if p.subpat.is_none()
                            && p.ident.to_string().starts_with(|c: char| c.is_lowercase()) =>
                    {
                        v.fact("pat_bind")
                    }
                    _ => {}
                }
                if arm.guard.is_some() {
                    v.fact("arm_guard");
                }
                v.visit_arm(arm);
            }
        });
    }

    fn visit_pat(&mut self, p: &'ast syn::Pat) {
        let path = match p {
            syn::Pat::Ident(i) if i.subpat.is_none() => {
                let s = i.ident.to_string();
                s.starts_with(|c: char| c.is_uppercase()).then_some(s)
            }
            syn::Pat::Path(e) => e.path.segments.last().map(|s| s.ident.to_string()),
            syn::Pat::TupleStruct(t) => t.path.segments.last().map(|s| s.ident.to_string()),
            syn::Pat::Struct(t) => t.path.segments.last().map(|s| s.ident.to_string()),
            syn::Pat::Or(_) => {
                self.fact("pat_or");
                None
            }
            syn::Pat::Range(r) => {
                self.fact("pat_range");
                if let syn::RangeLimits::Closed(_) = r.limits {
                    self.fact("pat_range_incl");
                }
                None
            }
            _ => None,
        };
        if let Some(name) = path {
            self.fact(format!("pat:{name}"));
        }
        // A path pattern's segments aren't expressions: skip `visit_expr_path`'s `ident:` fact.
        if !matches!(p, syn::Pat::Path(_)) {
            syn::visit::visit_pat(self, p);
        }
    }

    fn visit_expr_try(&mut self, t: &'ast syn::ExprTry) {
        self.fact("try");
        syn::visit::visit_expr_try(self, t);
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let ty = ty_text(&i.self_ty);
        self.fact(format!("impl:{ty}"));
        if let Some((_, t, _)) = &i.trait_
            && let Some(last) = t.segments.last()
        {
            self.fact("trait_impl");
            self.fact(format!("trait_impl:{}", last.ident));
        } else if !self.impls.insert(ty.clone()) {
            self.fact(format!("multi_impl:{ty}"));
        }
        // Methods of an `impl` without bounds sit inside `impl_unbounded` too.
        let bounded = i.generics.where_clause.is_some()
            || i.generics.params.iter().any(|p| match p {
                syn::GenericParam::Type(t) => !t.bounds.is_empty(),
                _ => false,
            });
        self.within(format!("impl:{ty}"), |v| {
            if bounded {
                syn::visit::visit_item_impl(v, i)
            } else {
                v.within("impl_unbounded", |v| syn::visit::visit_item_impl(v, i))
            }
        });
    }

    fn visit_expr_struct(&mut self, e: &'ast syn::ExprStruct) {
        let name = e
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        self.fact(format!("struct_lit:{name}"));
        self.within(format!("struct_lit:{name}"), |v| {
            for f in &e.fields {
                if f.colon_token.is_none()
                    && let syn::Member::Named(id) = &f.member
                {
                    v.fact(format!("shorthand:{id}"));
                }
            }
            if e.rest.is_some() {
                v.fact("update");
            }
            syn::visit::visit_expr_struct(v, e)
        });
    }

    fn visit_item_const(&mut self, c: &'ast syn::ItemConst) {
        let name = c.ident.to_string();
        self.visibility(&name, &c.vis);
        self.fact(format!("const:{name}"));
        self.fact(format!("const_ty:{}", ty_text(&c.ty)));
        self.within(format!("const:{name}"), |v| {
            syn::visit::visit_item_const(v, c)
        });
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        // The initialiser is visited inside `let:NAME`; the names are bound after it.
        let name = match &l.pat {
            syn::Pat::Ident(p) => Some(p.ident.to_string()),
            syn::Pat::Type(t) => match &*t.pat {
                syn::Pat::Ident(p) => Some(p.ident.to_string()),
                _ => None,
            },
            _ => None,
        };
        if let Some(init) = &l.init
            && let Some((_, diverge)) = &init.diverge
        {
            self.fact("let_else");
            self.visit_expr(&init.expr);
            self.within("let_else", |v| {
                v.visit_pat(&l.pat);
                v.visit_expr(diverge);
            });
        } else if let Some(init) = &l.init {
            match &name {
                Some(n) => self.within(format!("let:{n}"), |v| v.visit_local_init(init)),
                None => self.visit_local_init(init),
            }
        }
        self.pattern_lets(&l.pat);
        if let syn::Pat::Type(t) = &l.pat {
            self.visit_type(&t.ty);
        }
    }

    fn visit_expr_while(&mut self, e: &'ast syn::ExprWhile) {
        self.fact("while");
        if e.label.is_some() {
            self.fact("labeled_loop");
        }
        if has_let(&e.cond) {
            self.fact("while_let");
            self.within("while", |v| {
                v.within("while_let", |v| syn::visit::visit_expr_while(v, e))
            });
            return;
        }
        self.within("while", |v| syn::visit::visit_expr_while(v, e));
    }

    fn visit_expr_loop(&mut self, e: &'ast syn::ExprLoop) {
        self.fact("loop");
        if e.label.is_some() {
            self.fact("labeled_loop");
        }
        self.within("loop", |v| syn::visit::visit_expr_loop(v, e));
    }

    fn visit_expr_for_loop(&mut self, e: &'ast syn::ExprForLoop) {
        self.fact("for");
        let range = is_range(&e.expr);
        if range {
            self.fact("for_range");
        }
        if is_rev_range(&e.expr) {
            self.fact("for_rev");
        }
        if let syn::Pat::Reference(_) = &*e.pat {
            self.fact("for_ref");
        }
        if e.label.is_some() {
            self.fact("labeled_loop");
        }
        self.visit_expr(&e.expr);
        let ctx = if range { "for_range" } else { "for" };
        self.within("for", |v| {
            v.within(ctx, |v| {
                v.pattern_lets(&e.pat);
                v.visit_block(&e.body);
            })
        });
    }

    fn visit_expr_binary(&mut self, b: &'ast syn::ExprBinary) {
        let op = quote::ToTokens::to_token_stream(&b.op).to_string();
        self.fact(format!("op:{op}"));
        if let Some(plain) = op.strip_suffix('=')
            && matches!(plain, "+" | "-" | "*" | "/" | "%")
        {
            self.fact(format!("op:{plain}"));
            if let Expr::Path(p) = unparen(&b.left)
                && let Some(id) = p.path.get_ident()
            {
                self.fact(format!("assign:{id}"));
            }
        }
        syn::visit::visit_expr_binary(self, b);
    }

    fn visit_expr_assign(&mut self, a: &'ast syn::ExprAssign) {
        if let Expr::Path(p) = unparen(&a.left)
            && let Some(id) = p.path.get_ident()
        {
            self.fact(format!("assign:{id}"));
        }
        syn::visit::visit_expr_assign(self, a);
    }

    fn visit_expr_cast(&mut self, c: &'ast syn::ExprCast) {
        self.fact(format!("cast:{}", ty_text(&c.ty)));
        syn::visit::visit_expr_cast(self, c);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let Expr::Path(p) = unparen(&c.func)
            && let Some(last) = p.path.segments.last()
        {
            let name = last.ident.to_string();
            self.fact(format!("call:{name}"));
            if p.path.segments.len() > 1 {
                self.fact(format!("call_path:{}", path_text(&p.path)));
            }
            if p.path.segments.len() == 1
                && let Some(f) = self.current_fn()
            {
                self.calls.entry(f).or_default().insert(name);
            }
        }
        syn::visit::visit_expr_call(self, c);
    }

    fn visit_expr_method_call(&mut self, c: &'ast syn::ExprMethodCall) {
        self.fact(format!("call:{}", c.method));
        syn::visit::visit_expr_method_call(self, c);
    }

    fn visit_expr_path(&mut self, p: &'ast syn::ExprPath) {
        if let Some(last) = p.path.segments.last() {
            self.fact(format!("ident:{}", last.ident));
        }
        if p.path.segments.len() > 1 {
            self.fact(format!("path:{}", path_text(&p.path)));
        }
        syn::visit::visit_expr_path(self, p);
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        self.fact("if");
        if if_yields(i) {
            self.fact("if_expr");
        }
        if let Some((_, els)) = &i.else_branch
            && let Expr::If(_) = unparen(els)
        {
            self.fact("else_if");
        }
        if has_let(&i.cond) {
            self.fact("if_let");
            self.within("if", |v| {
                v.within("if_let", |v| {
                    v.visit_expr(&i.cond);
                    v.visit_block(&i.then_branch);
                });
                if let Some((_, els)) = &i.else_branch {
                    v.visit_expr(els);
                }
            });
            return;
        }
        self.within("if", |v| syn::visit::visit_expr_if(v, i));
    }

    fn visit_expr_block(&mut self, b: &'ast syn::ExprBlock) {
        let has_let = b.block.stmts.iter().any(|s| matches!(s, Stmt::Local(_)));
        if has_let && yields_value(&b.block) {
            self.fact("block_expr");
            self.within("block_expr", |v| syn::visit::visit_expr_block(v, b));
        } else {
            syn::visit::visit_expr_block(self, b);
        }
    }

    fn visit_expr_unary(&mut self, u: &'ast syn::ExprUnary) {
        if let syn::UnOp::Deref(_) = u.op {
            self.fact("deref");
        }
        syn::visit::visit_expr_unary(self, u);
    }

    fn visit_expr_field(&mut self, f: &'ast syn::ExprField) {
        let name = match &f.member {
            syn::Member::Named(id) => id.to_string(),
            syn::Member::Unnamed(i) => i.index.to_string(),
        };
        self.fact(format!("field_access:{name}"));
        syn::visit::visit_expr_field(self, f);
    }

    fn visit_expr_reference(&mut self, r: &'ast syn::ExprReference) {
        self.fact("borrow");
        if let Expr::Unary(u) = unparen(&r.expr)
            && let syn::UnOp::Deref(_) = u.op
        {
            self.fact(if r.mutability.is_some() {
                "reborrow_mut"
            } else {
                "reborrow"
            });
        }
        syn::visit::visit_expr_reference(self, r);
    }

    fn visit_expr_index(&mut self, i: &'ast syn::ExprIndex) {
        self.fact("index");
        syn::visit::visit_expr_index(self, i);
    }

    fn visit_expr_tuple(&mut self, t: &'ast syn::ExprTuple) {
        self.fact(format!("tuple:{}", t.elems.len()));
        syn::visit::visit_expr_tuple(self, t);
    }

    fn visit_lit_float(&mut self, _: &'ast syn::LitFloat) {
        self.fact("float");
    }

    fn visit_type_path(&mut self, t: &'ast syn::TypePath) {
        if t.path.is_ident("f64") || t.path.is_ident("f32") {
            self.fact("float");
        }
        for seg in &t.path.segments {
            self.fact(format!("type:{}", seg.ident));
        }
        syn::visit::visit_type_path(self, t);
    }

    fn visit_item(&mut self, i: &'ast syn::Item) {
        let kind = match i {
            syn::Item::Struct(_) => Some("struct"),
            syn::Item::Enum(_) => Some("enum"),
            syn::Item::Trait(_) => Some("trait"),
            syn::Item::Impl(_) => Some("impl"),
            syn::Item::Type(_) => Some("type"),
            syn::Item::Static(_) => Some("static"),
            syn::Item::Mod(_) => Some("mod"),
            syn::Item::Union(_) => Some("union"),
            syn::Item::Fn(_) => Some("fn"),
            syn::Item::Const(_) => Some("const"),
            syn::Item::Use(_) => Some("use"),
            _ => None,
        };
        if let Some(k) = kind {
            self.fact(format!("item:{k}"));
        }
        if let syn::Item::Fn(f) = i
            && f.sig.unsafety.is_some()
        {
            self.fact("unsafe");
        }
        syn::visit::visit_item(self, i);
    }

    fn visit_expr_unsafe(&mut self, u: &'ast syn::ExprUnsafe) {
        self.fact("unsafe");
        syn::visit::visit_expr_unsafe(self, u);
    }

    fn visit_lifetime(&mut self, l: &'ast syn::Lifetime) {
        self.fact("lifetime");
        self.fact(format!("lifetime:{}", l.ident));
    }

    fn visit_item_trait(&mut self, t: &'ast syn::ItemTrait) {
        let name = t.ident.to_string();
        self.visibility(&name, &t.vis);
        self.fact(format!("trait:{name}"));
        self.within(format!("trait:{name}"), |v| {
            syn::visit::visit_item_trait(v, t)
        });
    }

    fn visit_trait_item_fn(&mut self, f: &'ast syn::TraitItemFn) {
        let name = f.sig.ident.to_string();
        match &f.default {
            None => {
                self.fact(format!("required_fn:{name}"));
                syn::visit::visit_trait_item_fn(self, f);
            }
            Some(block) => {
                self.fact("item:fn");
                self.fact(format!("default_fn:{name}"));
                self.function(&f.sig, block, |v| syn::visit::visit_trait_item_fn(v, f));
            }
        }
    }

    fn visit_trait_bound(&mut self, b: &'ast syn::TraitBound) {
        if let Some(last) = b.path.segments.last() {
            self.fact("bound");
            self.fact(format!("bound:{}", last.ident));
        }
        syn::visit::visit_trait_bound(self, b);
    }

    fn visit_where_clause(&mut self, w: &'ast syn::WhereClause) {
        self.fact("where");
        syn::visit::visit_where_clause(self, w);
    }

    fn visit_type_trait_object(&mut self, t: &'ast syn::TypeTraitObject) {
        self.fact("dyn");
        syn::visit::visit_type_trait_object(self, t);
    }

    // Loop labels (`'outer:`, `break 'outer`) are lifetimes in syn, but not annotations.
    fn visit_label(&mut self, _: &'ast syn::Label) {}

    fn visit_expr_break(&mut self, b: &'ast syn::ExprBreak) {
        if b.label.is_some() {
            self.fact("labeled_break");
        }
        if b.expr.is_some() {
            self.fact("break_value");
        }
        if let Some(e) = &b.expr {
            self.visit_expr(e);
        }
    }

    fn visit_expr_continue(&mut self, _: &'ast syn::ExprContinue) {}

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        if let Some(last) = m.path.segments.last() {
            self.fact(format!("macro:{}", last.ident));
            // Assertions: the arguments past the checked ones are a custom message.
            let checked = match last.ident.to_string().trim_start_matches("debug_") {
                "assert" => Some(1),
                "assert_eq" | "assert_ne" => Some(2),
                _ => None,
            };
            if let Some(n) = checked {
                use syn::punctuated::Punctuated;
                let args = m
                    .parse_body_with(Punctuated::<Expr, syn::Token![,]>::parse_terminated)
                    .map_or(0, |a| a.len());
                self.fact(if args > n { "assert_msg" } else { "assert_bare" });
            }
        }
        // Arguments of `println!`/`format!` are expressions too.
        use syn::punctuated::Punctuated;
        if let Ok(args) = m.parse_body_with(Punctuated::<Expr, syn::Token![,]>::parse_terminated) {
            for a in &args {
                self.visit_expr(a);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facts(src: &str) -> Vec<Fact> {
        collect(&syn::parse_file(src).unwrap())
    }

    fn has(f: &[Fact], what: &str, inside: &[&str]) -> bool {
        f.iter()
            .any(|x| x.what == what && inside.iter().all(|c| x.ctx.iter().any(|y| y == c)))
    }

    #[test]
    fn loops_ops_and_contexts() {
        let f = facts(
            "pub fn digit_sum(mut n: u32) -> u32 { let mut s = 0; while n > 0 { s += n % 10; n /= 10; } s }
             fn fib(n: u32) -> u32 { let (mut a, mut b) = (0, 1); for _ in 0..n { let t = a + b; a = b; b = t; } a }
             fn down(n: u32) { for v in (1..=n).rev() { println!(\"{v:03}\", v = v as u64); } }",
        );
        assert!(has(&f, "while", &["fn:digit_sum"]));
        assert!(has(&f, "op:%", &["fn:digit_sum", "while"]));
        assert!(has(&f, "op:/", &["while"]));
        assert!(has(&f, "tail:s", &["fn:digit_sum"]));
        assert!(has(&f, "for_range", &["fn:fib"]));
        assert!(has(&f, "op:+", &["fn:fib", "for_range"]));
        assert!(has(&f, "let_tuple", &[]));
        assert!(has(&f, "for_rev", &["fn:down"]));
        assert!(has(&f, "cast:u64", &["fn:down", "for"]));
        assert!(!has(&f, "recursion", &[]));
    }

    #[test]
    fn values_and_recursion() {
        let f = facts(
            "const CAP: u8 = 255;
             fn a(n: u32) -> u32 { if n == 0 { 0 } else { b(n - 1) } }
             fn b(n: u32) -> u32 { a(n) }
             fn c(x: i32) -> i32 { let leap = { let r = x % 4; r == 0 }; let d = if leap { 1 } else if x > 0 { 2 } else { 3 }; d }
             fn d() -> u8 { let v = 'outer: loop { break 'outer CAP; }; v }",
        );
        assert!(has(&f, "recursion", &["fn:a"]));
        assert!(has(&f, "if_expr", &["fn:a"]));
        assert!(has(&f, "block_expr", &["fn:c", "let:leap"]));
        assert!(has(&f, "op:%", &["block_expr"]));
        assert!(has(&f, "else_if", &["fn:c"]));
        assert!(has(&f, "if_expr", &["let:d"]));
        assert!(has(&f, "labeled_loop", &["fn:d"]));
        assert!(has(&f, "labeled_break", &[]));
        assert!(has(&f, "break_value", &[]));
        assert!(has(&f, "ident:CAP", &["fn:d", "loop"]));
        assert!(has(&f, "const:CAP", &[]));
        assert!(has(&f, "const_ty:u8", &[]));
        assert!(!has(&f, "recursion", &["fn:d"]));
    }

    #[test]
    fn traits_bounds_and_lifetimes() {
        let f = facts(
            "pub trait Greet { fn name(&self) -> String; fn greeting(&self) -> String { format!(\"Hi {}\", self.name()) } }
             struct Pair<T> { a: T }
             impl<T: Clone + std::fmt::Display> Pair<T> { fn show(&self) -> String where T: PartialOrd { self.a.to_string() } }
             fn pick<'a>(x: &'a str, y: &'static str, d: impl Greet) -> Box<dyn std::error::Error> { todo!() }",
        );
        assert!(has(&f, "trait:Greet", &[]));
        assert!(has(&f, "required_fn:name", &["trait:Greet"]));
        assert!(has(&f, "default_fn:greeting", &["trait:Greet"]));
        assert!(has(&f, "call:name", &["trait:Greet", "fn:greeting"]));
        assert!(has(&f, "bound:Clone", &["impl:Pair<T>"]));
        assert!(has(&f, "bound:Display", &[]));
        assert!(has(&f, "bound:PartialOrd", &["fn:show"]));
        assert!(has(&f, "where", &["fn:show"]));
        assert!(!has(&f, "bound:Clone", &["struct:Pair"]));
        assert!(has(&f, "bound:Greet", &["fn:pick"]));
        assert!(has(&f, "lifetime:static", &["fn:pick"]));
        assert!(has(&f, "lifetime:a", &[]));
        assert!(has(&f, "dyn", &[]));
        assert!(has(&f, "field_access:a", &["fn:show"]));
        assert!(!has(&f, "fn:show", &["impl_unbounded"]));
        let h = facts("struct B<T> { e: Vec<T> } impl<T> B<T> { fn len(&self) -> usize { 0 } }");
        assert!(has(&h, "fn:len", &["impl_unbounded"]));
        let g = facts(
            "fn o(t: &str) -> impl Greet + use<> { todo!() }
             fn r<'a, T: Display>(a: &'a str, n: &T) -> impl View + use<'a, T> { let v = &a; *v; todo!() }",
        );
        assert!(has(&g, "ret:implGreet+use<>", &["fn:o"]));
        assert!(has(&g, "ret:implView+use<'a,T>", &["fn:r"]));
        assert!(has(&g, "borrow", &["fn:r", "let:v"]));
        assert!(has(&g, "deref", &["fn:r"]));
    }

    #[test]
    fn taught_subset_facts() {
        let f = facts(
            "struct P; fn a<'x>(s: &'x str) -> Vec<Option<&'x str>> { let v = vec![s]; 'l: loop { break 'l; } unsafe { } v.iter().map(|x| Some(*x)).collect() }",
        );
        for w in [
            "item:struct",
            "lifetime",
            "type:Vec",
            "type:Option",
            "macro:vec",
            "unsafe",
            "call:map",
            "call:collect",
            "labeled_break",
        ] {
            assert!(has(&f, w, &[]), "{w}");
        }
        let g = facts(
            "fn b(s: &str) -> usize { 'outer: for c in s.chars() { if c == ' ' { break 'outer; } } s.len() }",
        );
        assert!(!has(&g, "lifetime", &[]), "labels aren't lifetimes");
    }

    #[test]
    fn struct_facts() {
        let f = facts(
            "#[derive(Debug, Clone, PartialEq)] pub struct Book { pub id: u32, pub title: String }
             pub struct Mm(pub u32);
             struct Unit;
             impl Book { pub fn new(id: u32, title: &str) -> Book { let title = title.to_string(); Book { id, title } }
                         pub fn fork(&self) -> Book { Book { id: 1, ..self.clone() } } }
             impl Book { pub fn bump(&mut self) {} pub fn done(self) -> String { self.title } }",
        );
        assert!(has(&f, "derive:Debug", &["struct:Book"]));
        assert!(has(&f, "field:title:String", &["struct:Book"]));
        assert!(has(&f, "tuple_struct:Mm", &[]));
        assert!(has(&f, "unit_struct:Unit", &[]));
        assert!(has(&f, "multi_impl:Book", &[]));
        assert!(!has(&f, "trait_impl", &[]));
        let g = facts("struct A; impl A {} impl Clone for A { fn clone(&self) -> A { A } }");
        assert!(has(&g, "trait_impl:Clone", &[]));
        assert!(!has(&g, "multi_impl:A", &[]));
        assert!(has(&f, "recv:none", &["impl:Book", "fn:new"]));
        assert!(has(&f, "recv:ref", &["fn:fork"]));
        assert!(has(&f, "recv:mut", &["fn:bump"]));
        assert!(has(&f, "recv:own", &["fn:done"]));
        assert!(has(&f, "ret:()", &["fn:bump"]));
        assert!(has(&f, "ret:Book", &["fn:new"]));
        assert!(has(&f, "shorthand:id", &["fn:new", "struct_lit:Book"]));
        assert!(has(&f, "shorthand:title", &["fn:new", "struct_lit:Book"]));
        assert!(!has(&f, "shorthand:id", &["fn:fork"]));
        assert!(has(&f, "update", &["fn:fork", "struct_lit:Book"]));
    }

    #[test]
    fn test_attribute_facts() {
        let f = facts(
            "pub fn double(n: u16) -> u32 { n as u32 * 2 }
             #[cfg(test)]
             mod tests {
                 use super::*;
                 #[test] fn two() { assert_eq!(double(2), 4); }
                 #[test] #[should_panic(expected = \"zero\")] fn zero() { panic!(\"zero\") }
                 #[test] #[should_panic] fn any() { panic!() }
                 #[test] #[ignore = \"slow\"] fn slow() {}
                 fn helper() {}
             }
             #[test] fn outside() {}",
        );
        assert!(has(&f, "cfg_test", &[]));
        assert!(has(&f, "attr:test", &["cfg_test", "mod:tests"]));
        assert!(has(&f, "test_fn:two", &["cfg_test", "mod:tests"]));
        assert!(has(&f, "call:double", &["cfg_test", "mod:tests", "fn:two"]));
        assert!(has(&f, "should_panic_expected", &["cfg_test", "mod:tests"]));
        assert!(has(&f, "attr:should_panic", &["cfg_test", "mod:tests"]));
        assert!(has(&f, "ignore_reason", &["cfg_test", "mod:tests"]));
        assert!(!has(&f, "test_fn:helper", &[]));
        assert!(has(&f, "test_fn:outside", &[]));
        assert!(!has(&f, "test_fn:outside", &["cfg_test"]));
        assert_eq!(f.iter().filter(|x| x.what == "stray_test").count(), 1);
        assert_eq!(f.iter().filter(|x| x.what == "attr:test").count(), 5);
        assert_eq!(f.iter().filter(|x| x.what == "should_panic_expected").count(), 1);
        assert_eq!(f.iter().filter(|x| x.what == "should_panic_bare").count(), 1);
        let g = facts("fn t() { assert_eq!(1, 1, \"one is {}\", 1); assert!(true); assert_ne!(1, 2); assert!(1 < 2, \"why\"); }");
        assert_eq!(g.iter().filter(|x| x.what == "assert_msg").count(), 2);
        assert_eq!(g.iter().filter(|x| x.what == "assert_bare").count(), 2);
    }

    #[test]
    fn enum_and_pattern_facts() {
        let f = facts(
            "#[derive(Debug, Clone, Copy)] pub enum Coin { Penny, Move { x: i32, y: i32 }, Count(u32) }
             fn v(c: Coin) -> u32 { match c { Coin::Penny => 1, Coin::Move { x, .. } if x > 0 => 2, Coin::Count(n) => n, _ => 0 } }
             fn band(n: u8) -> &'static str { match n { 0 | 1 => \"low\", 2..=9 => \"mid\", other => \"x\" } }
             fn g(o: Option<u32>) -> Option<u32> { if let Some(n) = o { Some(n) } else { None } }
             fn h(a: Option<u32>, b: Option<&str>) -> Option<u32> {
                 let Some(x) = a else { return None; };
                 let Some(_) = b else { return None; };
                 let y = a?;
                 while let Some(z) = a { break; }
                 Some(x + y)
             }",
        );
        assert!(has(&f, "derive:Copy", &["enum:Coin"]));
        assert!(has(&f, "variant:Penny", &["enum:Coin"]));
        assert!(has(&f, "variant_shape:Move{x:i32,y:i32}", &["enum:Coin"]));
        assert!(has(&f, "variant_shape:Count(u32)", &["enum:Coin"]));
        assert!(has(&f, "match", &["fn:v"]));
        for p in [
            "pat:Penny",
            "pat:Move",
            "pat:Count",
            "pat_wild",
            "arm_guard",
        ] {
            assert!(has(&f, p, &["fn:v", "match"]), "{p}");
        }
        assert!(has(&f, "pat_or", &["fn:band", "match"]));
        assert!(has(&f, "pat_range_incl", &["fn:band", "match"]));
        assert!(has(&f, "pat_bind", &["fn:band", "match"]));
        assert!(!has(&f, "pat_wild", &["fn:band"]));
        assert!(has(&f, "if_let", &["fn:g"]));
        assert!(has(&f, "pat:Some", &["fn:g", "if_let"]));
        assert!(
            !has(&f, "pat:None", &["fn:g"]),
            "`None` here is an expression"
        );
        assert_eq!(
            f.iter()
                .filter(|x| x.what == "let_else" && x.ctx.contains(&"fn:h".to_string()))
                .count(),
            2
        );
        assert!(has(&f, "pat:Some", &["fn:h", "let_else"]));
        assert!(has(&f, "ident:None", &["fn:h", "let_else"]));
        assert!(has(&f, "try", &["fn:h"]));
        assert!(has(&f, "while_let", &["fn:h"]));
        assert!(has(&f, "pat:Some", &["while_let"]));
    }

    #[test]
    fn module_facts() {
        let f = facts(
            "pub mod fees;
             mod text;
             #[path = \"x.rs\"] mod odd;
             pub use fees::{charge, rate::{self, CENTS as C}};
             pub use text::*;
             const OFFSET: u64 = 7;
             pub mod reports {
                 pub(crate) fn absolute(n: u32) -> u64 { crate::arithmetic::double(n) + super::OFFSET }
                 fn hidden() {}
                 pub struct T { pub a: u8, b: u8 }
             }",
        );
        for m in [
            "mod_file:fees",
            "mod_file:text",
            "mod_decl:reports",
            "pub:fees",
            "attr:path",
        ] {
            assert!(has(&f, m, &[]), "{m}");
        }
        assert!(!has(&f, "pub:text", &[]));
        assert!(has(&f, "mod:reports", &[]));
        assert!(has(&f, "pub:absolute", &["mod:reports"]));
        assert!(has(&f, "pub_restricted:absolute", &["mod:reports"]));
        assert!(!has(&f, "pub:hidden", &[]));
        assert!(has(&f, "fn:hidden", &["mod:reports"]));
        assert!(has(&f, "pub_field:a", &["mod:reports", "struct:T"]));
        assert!(!has(&f, "pub_field:b", &[]));
        for u in [
            "pub_use:fees::charge",
            "pub_use:fees::rate",
            "pub_use:fees::rate::CENTS as C",
            "pub_use:text::*",
            "use_glob",
            "use_group",
        ] {
            assert!(has(&f, u, &[]), "{u}");
        }
        assert_eq!(f.iter().filter(|x| x.what == "item:use").count(), 2);
        assert!(has(
            &f,
            "call_path:crate::arithmetic::double",
            &["fn:absolute"]
        ));
        assert!(has(&f, "path:super::OFFSET", &["fn:absolute"]));
        assert!(has(&f, "lit:7", &[]));
        assert!(has(&f, "item:fn", &["mod:reports"]));
        let c = facts("fn f(v: &[i32]) -> usize { let g = |x: &i32| *x > 0; v.len() }");
        assert!(has(&c, "closure", &["fn:f"]));
    }

    #[test]
    fn statement_if_is_not_a_value() {
        let f = facts("fn m(x: i32) { if x > 0 { println!(\"a\"); } else { println!(\"b\"); } }");
        assert!(has(&f, "if", &[]));
        assert!(!has(&f, "if_expr", &[]));
    }
}
