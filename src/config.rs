use toml::Value;

pub struct CloneConfig {
    pub dest: String,
    pub url: String,
    pub force: bool,
}

impl CloneConfig {
    pub fn new(dest: Option<&str>, url: Option<&str>, force: bool) -> CloneConfig{
        let default_repo_name = String::from("dotfiles");
        let default_dest = match dirs::home_dir() {
            Some(mut k) => {
                k.push(default_repo_name);
                match k.to_str() {
                    Some(l) => String::from(l),
                    None => String::new(),
                }
            }
            None => String::new(),
        };
        CloneConfig {
            dest: match dest {
                Some(k) => String::from(k),
                None => default_dest,
            },
            url: match url {
                Some(k) => String::from(k),
                None => String::new(),
            },
            force: force,
        }
    }
}

pub struct LoadConfig {
    pub path: String,
    pub content: Option<Value>,
}

impl LoadConfig {
    pub fn new(path: Option<&str>) -> LoadConfig {
        let default_index_name = String::from("index.toml");
        let default_repo_name = String::from("dotfiles");
        let default_path = match dirs::home_dir() {
            Some(mut k) => {
                k.push(default_repo_name);
                k.push(default_index_name);
                match k.to_str() {
                    Some(l) => String::from(l),
                    None => String::new(),
                }
            }
            None => String::new(),
        };
        LoadConfig {
            path: match path {
                Some(k) => String::from(k),
                None => default_path,
            },
            content: None,
        }
    }
}

pub struct ViewConfig {
    pub load: Option<LoadConfig>,
}

pub struct LinkConfig {
    pub load: Option<LoadConfig>,
}

pub struct UnlinkConfig {
    pub load: Option<LoadConfig>,
}

pub struct Config {
    pub clone: Option<CloneConfig>,
    pub load: Option<LoadConfig>,
    pub view: Option<ViewConfig>,
    pub link: Option<LinkConfig>,
    pub unlink: Option<UnlinkConfig>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            clone: None,
            load: None,
            view: None,
            link: None,
            unlink: None,
        }
    }
}
