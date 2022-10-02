// debug 时关闭，release 时打开
// #![deny(warnings, missing_docs, unsafe_code)]
#![allow(unused)]

#[macro_use]
extern crate clap;
use clap::Parser;

// 常量定义
mod constants;
use constants::ENV_PATH;

// 编译环境信息
#[path = "actions/shadow.rs"]
mod shadow;

// proxy 代理设置
#[path = "actions/git_proxy.rs"]
mod git_proxy;
use git_proxy::ProxyArgs;

// qemu 模拟器
#[path = "actions/qemu.rs"]
mod qemu;
use qemu::QemuArgs;

// 错误定义
mod errors;

/// arch
mod arch;

#[derive(Parser)]
#[clap(name = "Xos Configure")]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// git proxy
    GitProxy(ProxyArgs),
    /// qemu
    Qemu(QemuArgs),
}

fn main() {
    // 解析环境变量
    // dotenv::dotenv().ok();
    dotenv::from_path(ENV_PATH).ok();

    // 设置 log
    log4rs::init_file(constants::CONFIG, Default::default()).unwrap();

    // 解析命令参数，调处理函数
    use Commands::*;
    match Cli::parse().command {
        GitProxy(proxy_args) => {
            if proxy_args.unset == true {
                proxy_args.unset();
            } else {
                proxy_args.set();
            }
            // shadow::shadow_config();
        }
        Qemu(qemu_args) => qemu_args.qemu(),
    }
}
