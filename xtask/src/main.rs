// #![deny(warnings, missing_docs, unsafe_code)]
#![allow(unused)]

#[macro_use]
extern crate clap;
use clap::Parser;

mod constant;

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
    // 解析环境变量
    dotenv::dotenv().ok();
    // 设置 log
    log4rs::init_file(constant::CONFIG, Default::default()).unwrap();
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
