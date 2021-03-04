use clap::{Arg, App};
mod command;
mod config;


fn parse_cli(config: &mut config::Config) {
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
                .about("Force file operations")
                .required(false)))
        .subcommand(App::new("load")
            .about("Load the index file in the repository")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .about("Specify the index file path")
                .required(false)))
        .subcommand(App::new("view")
            .about("Display the loaded configuration")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .about("Specify the index file path")
                .required(false)))
        .subcommand(App::new("link")
            .about("Create the links according to the loaded configuration")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .about("Specify the index file path")
                .required(false)))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("clone") {
        config.clone = Some(
            config::CloneConfig::new(
                matches.value_of("dest"),
                matches.value_of("url"),
                matches.is_present("force"),
            )
        )
    }
    if let Some(ref matches) = matches.subcommand_matches("load") {
        config.load = Some(
            config::LoadConfig::new(matches.value_of("path"))
        )
    }
    if let Some(ref matches) = matches.subcommand_matches("view") {
        config.view = Some(
            config::ViewConfig{
                load: Some(config::LoadConfig::new(matches.value_of("path")))
            }
        )
    }
    if let Some(_) = matches.subcommand_matches("link") {
        config.link = Some(
            config::LinkConfig{
                load: Some(config::LoadConfig::new(matches.value_of("path")))
            }
        )
    }
}

fn main() {
    let mut config = config::Config::new();
    parse_cli(&mut config);
    if let Some(k) = &config.clone {
        command::clone(k);
    }
    if let Some(k) = &mut config.load {
        command::load(k);
    }
    if let Some(k) = &mut config.view {
        command::view(k);
    }
}
