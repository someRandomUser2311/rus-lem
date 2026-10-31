pub trait Ranked {
    fn points(&self) -> i64;
    fn label(&self) -> &str;

    fn summary(&self) -> String {
        format!("{}: {}", self.label(), self.points())
    }
}

pub struct Player {
    pub name: String,
    pub points: i64,
}

impl Ranked for Player {
    fn points(&self) -> i64 {
        self.points
    }

    fn label(&self) -> &str {
        &self.name
    }
}

pub struct Leaderboard<T> {
    entries: Vec<T>,
}

impl<T> Leaderboard<T> {
    pub fn new() -> Self {
        Leaderboard {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: T) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl<T: Ranked> Leaderboard<T> {
    pub fn ranking(&self) -> Vec<&T> {
        let mut out: Vec<&T> = Vec::new();
        for entry in &self.entries {
            out.push(entry);
        }
        let mut i = 1;
        while i < out.len() {
            let mut j = i;
            while j > 0 && out[j].points() > out[j - 1].points() {
                out.swap(j, j - 1);
                j -= 1;
            }
            i += 1;
        }
        out
    }

    pub fn best(&self) -> Option<&T> {
        let mut best: Option<&T> = None;
        for entry in &self.entries {
            match best {
                Some(b) if b.points() >= entry.points() => {}
                _ => best = Some(entry),
            }
        }
        best
    }

    pub fn top(&self, limit: usize) -> Vec<&T> {
        let mut out = self.ranking();
        out.truncate(limit);
        out
    }

    pub fn view(&self, limit: usize) -> RankingView<'_, T> {
        RankingView {
            entries: self.top(limit),
        }
    }

    pub fn report(&self) -> String {
        self.view(usize::MAX).summary()
    }
}

pub struct RankingView<'a, T> {
    pub entries: Vec<&'a T>,
}

impl<'a, T> RankingView<'a, T> {
    pub fn first(&self) -> Option<&'a T> {
        if self.entries.is_empty() {
            None
        } else {
            Some(self.entries[0])
        }
    }
}

impl<'a, T: Ranked> RankingView<'a, T> {
    pub fn summary(&self) -> String {
        let mut out = String::new();
        for (i, entry) in self.entries.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            out.push_str(&format!("{}: {}", entry.label(), entry.points()));
        }
        out
    }
}
