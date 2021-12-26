pub struct AppSettings {
    music_path: String,
}

impl AppSettings {
    pub fn new(music_path: String) -> Self { Self { music_path } }

    /// Get a reference to the app settings's music path.
    pub fn music_path(&self) -> &str {
        self.music_path.as_ref()
    }
}
