mod cli;
mod config;
mod proxy;
mod scraper;
mod checker;
mod output;
mod enrich;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use scraper::Scraper;
use checker::Checker;
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    match cli.command {
        Commands::Check(mut args) => {
            // Load config and merge with args
            if let Ok(config) = Config::load(&cli.config) {
                config.merge_with_args(&mut args);
            }

            if args.sources.is_empty() {
                anyhow::bail!("No sources provided. Use --sources <URL/FILE> or specify in config.toml");
            }

            info!("Starting proxyhunt check...");
            
            let scraper = Scraper::new();
            let mut proxies = scraper.scrape_all(&args.sources).await;

            if let Some(max) = args.max {
                if proxies.len() > max {
                    proxies.truncate(max);
                }
            }
            
            if proxies.is_empty() {
                info!("No proxies found in sources.");
                return Ok(());
            }

            info!("Checking {} proxies...", proxies.len());
            let checker = Checker::new(args.clone());
            let successful = checker.check_all(proxies).await;

            info!("Found {} working proxies.", successful.len());

            output::save_results(
                &successful,
                &args.output,
                args.json.as_deref(),
                args.limit
            )?;
        }
        Commands::Quick(args) => {
            let sources = vec![
                "https://raw.githubusercontent.com/monosans/proxy-list/main/proxies/http.txt".to_string(),
                "https://raw.githubusercontent.com/monosans/proxy-list/main/proxies/socks4.txt".to_string(),
                "https://raw.githubusercontent.com/monosans/proxy-list/main/proxies/socks5.txt".to_string(),
                "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks5.txt".to_string(),
                "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/socks4.txt".to_string(),
                "https://raw.githubusercontent.com/TheSpeedX/SOCKS-List/master/http.txt".to_string(),
                "https://raw.githubusercontent.com/ShiftyTR/Proxy-List/master/proxy.txt".to_string(),
            ];

            info!("Starting quick proxyhunt check...");
            let scraper = Scraper::new();
            let mut proxies = scraper.scrape_all(&sources).await;

            if let Some(max) = args.max {
                if proxies.len() > max {
                    proxies.truncate(max);
                }
            }

            if proxies.is_empty() {
                info!("No proxies found.");
                return Ok(());
            }

            info!("Checking {} proxies...", proxies.len());
            
            // Map QuickArgs to CheckArgs for the checker
            let check_args = crate::cli::CheckArgs {
                http: true,
                socks4: true,
                socks5: true,
                sources,
                output: args.output.clone(),
                json: None,
                limit: args.limit,
                concurrency: args.concurrency,
                timeout: 10,
                connect_timeout: 5,
                max: args.max,
                check_url: "https://ipv4.icanhazip.com".to_string(),
                geoip_db: None,
                no_enrich: true,
            };

            let checker = Checker::new(check_args);
            let successful = checker.check_all(proxies).await;

            info!("Found {} working proxies.", successful.len());

            output::save_results(
                &successful,
                &args.output,
                None,
                args.limit
            )?;
        }
    }

    Ok(())
}
