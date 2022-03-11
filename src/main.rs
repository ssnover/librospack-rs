use clap::{Parser, Subcommand};
use librospack::*;

#[derive(Parser)]
struct RospackOptions {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    List,
}

fn main() {
    let options = RospackOptions::parse();

    match options.cmd {
        SubCommand::List => {
            println!("List called!");
            let packages = crawl(get_search_paths());
            for pkg in packages {
                println!("{} {}", pkg.name, pkg.path.to_string_lossy());
            }
        }
    }
}
