use std::fmt;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROXY_RE: Regex = Regex::new(r"(?i)(?:(?P<proto>https?|socks[45])://)?(?:(?P<user>[^:]+):(?P<pass>[^@]+)@)?(?P<host>[a-z0-9.-]+):(?P<port>\d+)").unwrap();
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProxyProto {
    Http,
    Socks4,
    Socks5,
}

impl fmt::Display for ProxyProto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyProto::Http => write!(f, "http"),
            ProxyProto::Socks4 => write!(f, "socks4"),
            ProxyProto::Socks5 => write!(f, "socks5"),
        }
    }
}

impl FromStr for ProxyProto {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "http" | "https" => Ok(ProxyProto::Http),
            "socks4" => Ok(ProxyProto::Socks4),
            "socks5" => Ok(ProxyProto::Socks5),
            _ => Err(anyhow!("Invalid protocol: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub proto: ProxyProto,
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub pass: Option<String>,
}

impl Proxy {
    pub fn new(proto: ProxyProto, host: &str, port: u16) -> Self {
        Self {
            proto,
            host: host.to_string(),
            port,
            user: None,
            pass: None,
        }
    }

    pub fn full_url(&self) -> String {
        let auth = match (&self.user, &self.pass) {
            (Some(u), Some(p)) => format!("{}:{}@", u, p),
            _ => String::new(),
        };
        format!("{}://{}{}:{}", self.proto, auth, self.host, self.port)
    }

    pub fn parse_multiple(text: &str) -> Vec<Self> {
        let mut proxies = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Ok(proxy) = line.parse::<Proxy>() {
                proxies.push(proxy);
            }
        }
        proxies
    }
}

impl FromStr for Proxy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let caps = PROXY_RE.captures(s).ok_or_else(|| anyhow!("Invalid proxy format: {}", s))?;
        
        let proto = caps.name("proto")
            .map(|m| ProxyProto::from_str(m.as_str()))
            .transpose()?
            .unwrap_or(ProxyProto::Http); // Default to HTTP

        let host = caps.name("host").unwrap().as_str().to_string();
        let port = caps.name("port").unwrap().as_str().parse::<u16>()?;
        let user = caps.name("user").map(|m| m.as_str().to_string());
        let pass = caps.name("pass").map(|m| m.as_str().to_string());

        Ok(Proxy {
            proto,
            host,
            port,
            user,
            pass,
        })
    }
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_url())
    }
}
