use anyhow::Result;
use reqwest::Client;
use std::fs;
use crate::proxy::Proxy;
use tracing::{info, warn};

pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("proxyhunt/0.1.0")
                .build()
                .unwrap(),
        }
    }

    pub async fn scrape_all(&self, sources: &[String]) -> Vec<Proxy> {
        let mut all_proxies = Vec::new();
        
        for source in sources {
            match self.scrape_source(source).await {
                Ok(proxies) => all_proxies.extend(proxies),
                Err(e) => warn!("Failed to scrape source {}: {}", source, e),
            }
        }

        let unique_proxies = all_proxies.collect_unique();
        info!("Scraped total {} unique proxies", unique_proxies.len());
        unique_proxies
    }

    pub async fn scrape_source(&self, source: &str) -> Result<Vec<Proxy>> {
        let text = if source.starts_with("http://") || source.starts_with("https://") {
            self.client.get(source).send().await?.text().await?
        } else {
            fs::read_to_string(source)?
        };

        Ok(Proxy::parse_multiple(&text))
    }
}

trait UniqueExt {
    fn collect_unique(self) -> Vec<Proxy>;
}

impl UniqueExt for Vec<Proxy> {
    fn collect_unique(self) -> Vec<Proxy> {
        let mut unique = std::collections::HashSet::new();
        let mut result = Vec::new();
        for p in self {
            let key = (p.proto, p.host.clone(), p.port, p.user.clone(), p.pass.clone());
            if unique.insert(key) {
                result.push(p);
            }
        }
        result
    }
}
