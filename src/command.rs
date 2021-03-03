use git2::Repository;
use crate::config;

pub fn clone(config: config::CloneConfig) {
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
}
