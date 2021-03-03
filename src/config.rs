pub struct CloneConfig {
    pub dest: String,
    pub url: String,
    pub force: bool,
}

pub struct Config {
    pub clone: Option<CloneConfig>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            clone: None,
        }
    }
}
