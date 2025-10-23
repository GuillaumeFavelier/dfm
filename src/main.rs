use clap::{Arg, App};
mod command;
mod config;


fn parse_cli(config: &mut config::Config) {
    let matches = App::new("Dotfiles Manager")
        .subcommand(App::new("view")
            .about("Display the loaded configuration")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .about("Specify the index file path")
                .required(false)))
        .subcommand(App::new("link")
            .about("Create the links described in the configuration")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .about("Specify the index file path")
                .required(false)))
        .subcommand(App::new("unlink")
            .about("Remove the links described in the configuration")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .about("Specify the index file path")
                .required(false)))
        .get_matches();

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
    if let Some(_) = matches.subcommand_matches("unlink") {
        config.unlink = Some(
            config::UnlinkConfig{
                load: Some(config::LoadConfig::new(matches.value_of("path")))
            }
        )
    }
}

fn main() {
    let mut config = config::Config::new();
    parse_cli(&mut config);
    if let Some(k) = &mut config.view {
        command::view(k);
    }
    if let Some(k) = &mut config.link {
        command::link(k);
    }
    if let Some(k) = &mut config.unlink {
        command::unlink(k);
    }
}
