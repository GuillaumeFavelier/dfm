use toml::Value;
use shellexpand::{self};
use std::os::unix::fs;

pub fn load(path: &String) -> Option<Value> {
    let default_tag = "linux";
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => panic!("Failed to read {}: {}", path, e)
    };
    let parsed = match content.parse::<Value>() {
        Ok(c) => c,
        Err(e) => panic!("Failed to parse {}: {}", path, e)
    };
    let content = parsed.get(default_tag);
    println!("Loaded {}", path);
    content.cloned()
}

pub fn view(path: &String) {
    let content = load(path);
    let content = match content {
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

pub fn link(path: &String) {
    let content = load(path);
    let content = match content {
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
            src_path.push(path);
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

pub fn unlink(path: &String) {
    let content = load(path);
    let content = match content {
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
