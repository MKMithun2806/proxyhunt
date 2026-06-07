use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;
use crate::cli::CheckArgs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub check: CheckConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckConfig {
    pub http: Option<bool>,
    pub socks4: Option<bool>,
    pub socks5: Option<bool>,
    pub sources: Option<Vec<String>>,
    pub output: Option<String>,
    pub json: Option<String>,
    pub limit: Option<usize>,
    pub concurrency: Option<usize>,
    pub timeout: Option<u64>,
    pub connect_timeout: Option<u64>,
    pub check_url: Option<String>,
    pub geoip_db: Option<String>,
    pub no_enrich: Option<bool>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn merge_with_args(&self, args: &mut CheckArgs) {
        if let Some(v) = self.check.http { if !args.http { args.http = v; } }
        if let Some(v) = self.check.socks4 { if !args.socks4 { args.socks4 = v; } }
        if let Some(v) = self.check.socks5 { if !args.socks5 { args.socks5 = v; } }
        if let Some(ref v) = self.check.sources { if args.sources.is_empty() { args.sources = v.clone(); } }
        if let Some(ref v) = self.check.output { if args.output == "proxies.txt" { args.output = v.clone(); } }
        if let Some(ref v) = self.check.json { if args.json.is_none() { args.json = Some(v.clone()); } }
        if let Some(v) = self.check.limit { if args.limit.is_none() { args.limit = Some(v); } }
        if let Some(v) = self.check.concurrency { if args.concurrency == 512 { args.concurrency = v; } }
        if let Some(v) = self.check.timeout { if args.timeout == 10 { args.timeout = v; } }
        if let Some(v) = self.check.connect_timeout { if args.connect_timeout == 5 { args.connect_timeout = v; } }
        if let Some(ref v) = self.check.check_url { if args.check_url == "https://ipv4.icanhazip.com" { args.check_url = v.clone(); } }
        if let Some(ref v) = self.check.geoip_db { if args.geoip_db.is_none() { args.geoip_db = Some(v.clone()); } }
        if let Some(v) = self.check.no_enrich { if !args.no_enrich { args.no_enrich = v; } }
    }
}
