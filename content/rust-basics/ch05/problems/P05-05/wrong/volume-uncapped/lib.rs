pub struct Settings {
    pub theme: String,
    pub brightness: u8,
    pub volume: u8,
    pub font_size: u8,
    pub notifications: bool,
}

pub fn night_mode(base: Settings) -> Settings {
    Settings {
        theme: String::from("dark"),
        brightness: base.brightness.min(30),
        notifications: false,
        ..base
    }
}

pub fn with_volume(base: &Settings, volume: u8) -> Settings {
    Settings {
        theme: base.theme.clone(),
        volume,
        ..*base
    }
}
