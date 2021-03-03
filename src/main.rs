use git2::Repository;
use clap::{Arg, App};

struct Config {
    dest: String,
    url: String,
    force: bool,
}

fn get_default_config() -> Config {
    let default_dest = match dirs::home_dir() {
        Some(mut i) => {
            i.push("dotfiles");
            match i.to_str() {
                Some(j) => String::from(j),
                None => String::new(),
            }
        },
        None => String::new(),
    };
    Config {
        dest: default_dest,
        url: String::new(),
        force: false,
    }
}

fn clone(config: Config) {
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

fn parse_cli(config: &mut Config) {
    let matches = App::new("Dotfiles Manager")
        .subcommand(App::new("clone")
            .about("Clone dotfiles repository from remote")
            .arg(Arg::new("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .about("Specify GitHub HTTPS remote")
                .required(true))
            .arg(Arg::new("dest")
                .short('d')
                .long("dest")
                .value_name("DEST")
                .about("Specify the destination path")
                .required(false))
            .arg(Arg::new("force")
                .short('f')
                .long("force")
                .about("Force file operations.")
                .required(false)))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("clone") {
        if let Some(k) = matches.value_of("url") {
            config.url = String::from(k); 
        }
        if let Some(k) = matches.value_of("dest") {
            config.dest = String::from(k); 
        }
        config.force = matches.is_present("force");
    }
}

fn main() {
    let mut config = get_default_config();
    parse_cli(&mut config);
    clone(config);
}
