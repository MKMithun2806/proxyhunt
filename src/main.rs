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
            let proxies = scraper.scrape_all(&args.sources).await;
            
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
    }

    Ok(())
}
