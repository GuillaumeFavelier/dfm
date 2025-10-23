use clap::Parser;
mod command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Display the loaded configuration
    #[arg(short, long, value_name = "FILE")]
    view: Option<String>,

    /// Create the links described in the configuration
    #[arg(short, long, value_name = "FILE")]
    link: Option<String>,

    /// Remove the links described in the configuration
    #[arg(short, long, value_name = "FILE")]
    unlink: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(p) = cli.view.as_deref() {
        command::view(&p.to_string());
    }
    if let Some(p) = cli.link.as_deref() {
        command::link(&p.to_string());
    }
    if let Some(p) = cli.unlink.as_deref() {
        command::unlink(&p.to_string());
    }
}
