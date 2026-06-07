use std::time::{Duration, Instant};
use anyhow::Result;
use reqwest::{Client, Proxy as ReqwestProxy};
use crate::proxy::{Proxy, ProxyProto};
use crate::cli::CheckArgs;
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CheckedProxy {
    pub proxy: Proxy,
    pub latency: Duration,
    pub status: bool,
}

pub struct Checker {
    args: CheckArgs,
}

impl Checker {
    pub fn new(args: CheckArgs) -> Self {
        Self { args }
    }

    pub async fn check_all(&self, proxies: Vec<Proxy>) -> Vec<CheckedProxy> {
        let pb = ProgressBar::new(proxies.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"));

        let results = futures::stream::iter(proxies)
            .map(|p| {
                let args = self.args.clone();
                let pb = pb.clone();
                async move {
                    let res = self.check_proxy(p, &args).await;
                    pb.inc(1);
                    res
                }
            })
            .buffer_unordered(self.args.concurrency)
            .collect::<Vec<CheckedProxy>>()
            .await;

        pb.finish_with_message("Done");

        let mut successful: Vec<CheckedProxy> = results.into_iter().filter(|r| r.status).collect();
        successful.sort_by_key(|r| r.latency);
        successful
    }

    async fn check_proxy(&self, proxy: Proxy, args: &CheckArgs) -> CheckedProxy {
        let start = Instant::now();
        let status = self.test_connection(&proxy, args).await.is_ok();
        let latency = start.elapsed();

        CheckedProxy {
            proxy,
            latency,
            status,
        }
    }

    async fn test_connection(&self, proxy: &Proxy, args: &CheckArgs) -> Result<()> {
        let proxy_url = proxy.full_url();
        let reqwest_proxy = match proxy.proto {
            ProxyProto::Http => ReqwestProxy::all(&proxy_url)?,
            ProxyProto::Socks4 => ReqwestProxy::all(&proxy_url)?, // Reqwest handles socks4://
            ProxyProto::Socks5 => ReqwestProxy::all(&proxy_url)?, // Reqwest handles socks5://
        };

        let client = Client::builder()
            .proxy(reqwest_proxy)
            .timeout(Duration::from_secs(args.timeout))
            .connect_timeout(Duration::from_secs(args.connect_timeout))
            .danger_accept_invalid_certs(true)
            .build()?;

        let resp = client.get(&args.check_url).send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Status: {}", resp.status()))
        }
    }
}
