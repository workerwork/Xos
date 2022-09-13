use std::{env, fs, net::Ipv4Addr};

use cmd_lib::{run_cmd, run_fun};

use super::constants::{DNS_CONF, OUTPUT, PROXY_PORT, PROXY_TYPE};

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
            env::var(PROXY_PORT)
                .expect("PROXY_PORT must be set")
                .parse::<u16>()
                .unwrap()
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
        if let Err(e) = run_cmd!(git config --global http.proxy $proxy >> $OUTPUT) {
            log::error!("{e}");
        }
        let proxy = format!("{PROXY_TYPE}s://{dns}:{port}");
        if let Err(e) = run_cmd!(git config --global https.proxy $proxy >> $OUTPUT) {
            log::error!("{e}");
        }
    }

    /// unset git proxy
    pub(crate) fn unset(&self) {
        if let Err(e) = run_cmd!(
            git config --global --unset http.proxy >> $OUTPUT;
            git config --global --unset https.proxy >> $OUTPUT;
        ) {
            log::error!("{e}");
        }
    }
}
