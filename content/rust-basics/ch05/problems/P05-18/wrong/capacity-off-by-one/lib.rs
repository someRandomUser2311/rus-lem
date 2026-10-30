#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    pub name: String,
    pub songs: [u32; 5],
    pub len: usize,
}

impl Playlist {
    pub fn new(name: &str) -> Playlist {
        Playlist {
            name: name.to_string(),
            songs: [0; 5],
            len: 0,
        }
    }

    /// Adds a song id at the end. Returns false (and changes nothing) when there are already 5 songs.
    pub fn add(&mut self, id: u32) -> bool {
        if self.len > 5 {
            return false;
        }
        self.songs[self.len] = id;
        self.len += 1;
        true
    }

    /// Removes every song.
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Number of songs.
    pub fn count(&self) -> usize {
        self.len
    }

    /// Gives the playlist a new name and returns the old name.
    pub fn rename(&mut self, name: &str) -> String {
        let old = self.name.clone();
        self.name = name.to_string();
        old
    }

    /// Consumes the playlist and returns "<name>: <count> song(s)".
    pub fn summary(self) -> String {
        format!("{}: {} song(s)", self.name, self.len)
    }
}

pub fn demo() -> String {
    let mut list = Playlist::new("Road trip");
    list.add(7);
    list.add(12);
    let n = list.count();
    let text = list.summary();
    format!("{text} [{n}]")
}
