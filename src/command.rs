use git2::Repository;
use toml::Value;
use shellexpand::{self};
use std::os::unix::fs;
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
    let load_config = match &mut config.load {
        Some(i) => i,
        None => panic!("No configuration is loaded."),
    };
    load(load_config);
    let content = match &mut load_config.content {
        Some(j) => j,
        None => panic!("No content is found."),
    };
    if !content.is_table() {
        return;
    }
    if let Value::Table(t) = content {
        for (src, dst) in t {
            println!("{} {}", src, dst);
        }
    }
}

pub fn link(config: &mut config::LinkConfig) {
    let load_config = match &mut config.load {
        Some(i) => i,
        None => panic!("No configuration is loaded."),
    };
    load(load_config);
    let content = match &mut load_config.content {
        Some(j) => j,
        None => panic!("No content is found."),
    };
    if !content.is_table() {
        return;
    }
    if let Value::Table(t) = content {
        for (src, dst) in t {
            let mut str_dst = shellexpand::full(dst.as_str().unwrap()).unwrap();
            let mut src_path = std::path::PathBuf::new();
            src_path.push(&load_config.path);
            src_path.pop();
            src_path.push(src.as_str());
            match fs::symlink(src_path, str_dst.to_mut()) {
                Ok(_) => (),
                Err(_) => {
                    println!("{} cannot be linked.", src);
                    continue
                }
            }
        }
    }
    println!("Links are created.");
}

pub fn unlink(config: &mut config::UnlinkConfig) {
    let load_config = match &mut config.load {
        Some(i) => i,
        None => panic!("No configuration is loaded."),
    };
    load(load_config);
    let content = match &mut load_config.content {
        Some(j) => j,
        None => panic!("No content is found."),
    };
    if !content.is_table() {
        return;
    }
    if let Value::Table(t) = content {
        for (src, dst) in t {
            let mut str_dst = shellexpand::full(dst.as_str().unwrap()).unwrap();
            match std::fs::remove_file(str_dst.to_mut()) {
                Ok(_) => (),
                Err(_) => {
                    println!("{} cannot be unlinked.", src);
                    continue
                }
            }
        }
    }
    println!("Links are removed.");
}
