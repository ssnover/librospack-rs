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
            let mut packages = crawl(get_search_paths());
            packages.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap());
            for pkg in packages {
                if !pkg.is_metapackage {
                    println!("{} {}", pkg.name, pkg.path.to_string_lossy());
                }
            }
        }
    }
}
