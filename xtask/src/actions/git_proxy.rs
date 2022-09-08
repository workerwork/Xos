#[path = "../constant.rs"]
mod constant;
use constant::PROXY_PORT;

use std::env;
use std::{fs, net::Ipv4Addr};

#[derive(Args)]
pub(crate) struct Proxy {
    #[clap(short, long)]
    port: Option<u16>,
    #[clap(short, long)]
    global: bool,
    #[clap(long)]
    pub unset: bool,
}

impl Proxy {
    /// set git proxy
    pub(crate) fn set(&self) {
        let s = env::var(PROXY_PORT).expect("PROXY_PORT must be set");
        if let Some(port) = self.port {
            println!("port: {port}");
        } else {
            println!("port: {s}");
        }

        let path = "/etc/resolv.conf".to_string();
        let dns = fs::read_to_string(path)
            .unwrap()
            .lines()
            .find_map(|line| {
                line.strip_prefix("nameserver ")
                    .and_then(|s| s.parse::<Ipv4Addr>().ok())
            })
            .expect("FAILED: detect DNS");
        // let _proxy = format!("http://{}:{}", dns, self.port);
    }

    /// unset git proxy
    pub(crate) fn unset(&self) {
        println!("unset");
    }
}
