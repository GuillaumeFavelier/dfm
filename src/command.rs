use git2::Repository;
use toml::Value;
use crate::config;

pub fn clone(config: &config::CloneConfig) {
    let path = std::path::Path::new(&config.dest);
    if path.exists() {
        if config.force {
            match std::fs::remove_dir_all(path) {
                Ok(_) => (),
                Err(e) => panic!("Failed to replace: {}", e),
            }
        }
        else {
            panic!("Path already exists: {}", path.display());
        }
    }
    match Repository::clone(&config.url, path) {
        Ok(_) => (),
        Err(e) => panic!("Failed to clone: {}", e),
    }
    println!("Cloned {} to {}", config.url, config.dest);
}

pub fn load(config: &mut config::LoadConfig) {
    let default_tag = "linux";
    let content = match std::fs::read_to_string(&config.path) {
        Ok(c) => c,
        Err(e) => panic!("Failed to load {}: {}", config.path, e)
    };
    let parsed = match content.parse::<Value>() {
        Ok(c) => c,
        Err(e) => panic!("Failed to parse {}: {}", config.path, e)
    };
    if let Some(k) = parsed.get(default_tag) {
        config.content = Some(k.clone());
    }
    println!("Loaded {}", config.path);
}

pub fn view(config: &mut config::ViewConfig) {
    match &mut config.load {
        Some(i) => {
            load(i);
            match &mut i.content {
                Some(j) => {
                    if j.is_table() {
                        if let Value::Table(l) = j {
                            for (src, dst) in l {
                                println!("{} {}", src, dst);
                            }
                        }
                    }
                }
                None => panic!("No content is found"),
            }
        }
        None => panic!("No configuration is loaded."),
    }
}
