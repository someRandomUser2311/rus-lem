#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(pub u32);

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl From<u32> for Id {
    fn from(n: u32) -> Self {
        Id(n)
    }
}

impl From<Id> for u32 {
    fn from(id: Id) -> u32 {
        id.0
    }
}

pub fn ordered_ids(values: &[u32]) -> Vec<String> {
    let mut ids = Vec::new();
    for n in values {
        ids.push(Id::from(*n));
    }
    ids.sort();
    ids.dedup();
    let mut out = Vec::new();
    for id in ids {
        out.push(id.to_string());
    }
    out
}
