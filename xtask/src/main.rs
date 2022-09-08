// #![deny(warnings, missing_docs, unsafe_code)]
#[macro_use]
extern crate clap;
use clap::Parser;


#[path = "actions/shadow.rs"]
mod shadow;

#[path = "actions/git_proxy.rs"]
mod git_proxy;
use git_proxy::Proxy;

#[derive(Parser)]
#[clap(name = "Xos Configure")]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GitProxy(Proxy),
}

fn main() {
    dotenv::dotenv().ok();
    use Commands::*;
    match Cli::parse().command {
        GitProxy(proxy) => {
            if proxy.unset == true {
                proxy.unset();
            } else {
                proxy.set();
            }
            // shadow::shadow_config();
        }
    }
}
