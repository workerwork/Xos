use std::{env, fs, net::Ipv4Addr};

use super::constants::{DNS_CONF, OUTPUT, PROXY_TYPE};

#[derive(Args)]
pub(crate) struct ProxyArgs {
    #[clap(short, long)]
    port: Option<u16>,
    #[clap(long)]
    pub unset: bool,
}

impl ProxyArgs {
    /// set git proxy
    pub(crate) fn set(&self) {
        let port = if let Some(port) = self.port {
            port
        } else {
            env::var("PROXY_PORT")
                .expect("proxy port must be set")
                .parse::<u16>()
                .expect("proxy port parse error!")
        };
        let dns = fs::read_to_string(DNS_CONF)
            .unwrap()
            .lines()
            .find_map(|line| {
                line.strip_prefix("nameserver ")
                    .and_then(|s| s.parse::<Ipv4Addr>().ok())
            })
            .expect("FAILED: detect DNS");
        let proxy = format!("{PROXY_TYPE}://{dns}:{port}");
        cmd_lib::run_cmd!(git config --global http.proxy $proxy >> $OUTPUT)
            .map(|_| println!("git proxy: {proxy}"))
            .unwrap();
        let proxy = format!("{PROXY_TYPE}s://{dns}:{port}");
        cmd_lib::run_cmd!(git config --global https.proxy $proxy >> $OUTPUT)
            .map(|_| println!("git proxy: {proxy}"))
            .unwrap();
    }

    /// unset git proxy
    pub(crate) fn unset(&self) {
        if let Err(e) = cmd_lib::run_cmd!(
            git config --global --unset http.proxy >> $OUTPUT;
            git config --global --unset https.proxy >> $OUTPUT;
        ) {
            log::error!("{e}");
        }
    }
}
