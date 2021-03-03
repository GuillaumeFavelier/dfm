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
                .required(true))
            .arg(Arg::new("force")
                .short('f')
                .long("force")
                .about("Force file operations.")
                .required(false)))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("clone") {
        config.clone = Some(
            config::CloneConfig {
                url: match matches.value_of("url") {
                    Some(k) => String::from(k),
                    None => String::new(),
                },
                dest: match matches.value_of("dest") {
                    Some(k) => String::from(k),
                    None => String::new(),
                },
                force: matches.is_present("force")
            }
        )
    }
}

fn main() {
    let mut config = config::Config::new();
    parse_cli(&mut config);
    match config.clone {
        Some(c) => command::clone(c),
        None => (),
    }
}
