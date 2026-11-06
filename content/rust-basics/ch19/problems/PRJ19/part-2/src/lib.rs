#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set { key: String, value: i64 },
    Get(String),
    Del(String),
    Incr { key: String, by: i64 },
    List,
    Count { lo: i64, hi: i64 },
}

fn number(word: &str) -> Result<i64, String> {
    word.parse::<i64>().map_err(|_| format!("bad number: {word}"))
}

pub fn parse(line: &str) -> Result<Cmd, String> {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        [] => Err(String::from("empty command")),
        ["set", key, value] => Ok(Cmd::Set {
            key: key.to_string(),
            value: number(value)?,
        }),
        ["get", key] => Ok(Cmd::Get(key.to_string())),
        ["del" | "delete", key] => Ok(Cmd::Del(key.to_string())),
        ["incr", key] => Ok(Cmd::Incr {
            key: key.to_string(),
            by: 1,
        }),
        ["incr", key, by] => Ok(Cmd::Incr {
            key: key.to_string(),
            by: number(by)?,
        }),
        ["list"] => Ok(Cmd::List),
        ["count", lo, "to", hi] => Ok(Cmd::Count {
            lo: number(lo)?,
            hi: number(hi)?,
        }),
        [cmd @ ("set" | "get" | "del" | "delete" | "incr" | "list" | "count"), ..] => {
            Err(format!("wrong arguments for {cmd}"))
        }
        [other, ..] => Err(format!("unknown command: {other}")),
    }
}

fn size(v: i64) -> &'static str {
    match v {
        0 => "zero",
        1..=9 => "small",
        n if n < 0 => "negative",
        _ => "large",
    }
}

#[derive(Debug, Default)]
pub struct Store {
    entries: Vec<(String, i64)>,
}

impl Store {
    pub fn new() -> Store {
        Store::default()
    }

    fn get(&self, key: &str) -> Option<i64> {
        self.entries
            .iter()
            .find(|(k, _)| k == key)
            .map(|&(_, v)| v)
    }

    fn set(&mut self, key: String, value: i64) {
        match self.entries.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => *v = value,
            None => self.entries.push((key, value)),
        }
    }

    pub fn execute(&mut self, cmd: Cmd) -> String {
        match cmd {
            Cmd::Set { key, value } => {
                self.set(key, value);
                String::from("OK")
            }
            Cmd::Get(key) => match self.get(&key) {
                Some(v) => format!("{v} ({})", size(v)),
                None => String::from("(none)"),
            },
            Cmd::Del(key) => match self.entries.iter().position(|(k, _)| *k == key) {
                Some(i) => {
                    self.entries.remove(i);
                    String::from("deleted")
                }
                None => String::from("(none)"),
            },
            Cmd::Incr { key, by } => {
                let v = self.get(&key).unwrap_or(0).saturating_add(by);
                self.set(key, v);
                v.to_string()
            }
            Cmd::List if self.entries.is_empty() => String::from("(empty)"),
            Cmd::List => self
                .entries
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(" "),
            Cmd::Count { lo, hi } => self
                .entries
                .iter()
                .filter(|&&(_, v)| (lo..=hi).contains(&v))
                .count()
                .to_string(),
        }
    }
}

pub fn run(script: &str) -> String {
    let mut store = Store::new();
    let mut out = String::new();
    for line in script.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let response = match parse(line) {
            Ok(cmd) => store.execute(cmd),
            Err(e) => format!("error: {e}"),
        };
        out.push_str(&response);
        out.push('\n');
    }
    out
}
