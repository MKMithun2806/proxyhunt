use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "proxyhunt")]
#[command(about = "A fast, modern proxy scraper and checker", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scrape and check proxies
    Check(CheckArgs),
    /// Quick scrape and check using built-in sources
    Quick(QuickArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct CheckArgs {
    /// Enable HTTP proxies
    #[arg(long)]
    pub http: bool,

    /// Enable SOCKS4 proxies
    #[arg(long)]
    pub socks4: bool,

    /// Enable SOCKS5 proxies
    #[arg(long)]
    pub socks5: bool,

    /// Sources to scrape from (URLs or file paths)
    #[arg(short, long)]
    pub sources: Vec<String>,

    /// Output file path
    #[arg(short, long, default_value = "proxies.txt")]
    pub output: String,

    /// Output JSON file path
    #[arg(long)]
    pub json: Option<String>,

    /// Limit output to top N fastest proxies
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// Maximum concurrent checks
    #[arg(short, long, default_value_t = 512)]
    pub concurrency: usize,

    /// Total timeout for each check in seconds
    #[arg(short, long, default_value_t = 10)]
    pub timeout: u64,

    /// Connection timeout for each check in seconds
    #[arg(long, default_value_t = 5)]
    pub connect_timeout: u64,

    /// Maximum number of proxies to check
    #[arg(long)]
    pub max: Option<usize>,

    /// URL to check proxies against
    #[arg(long, default_value = "https://ipv4.icanhazip.com")]
    pub check_url: String,

    /// Path to MaxMind GeoIP2 database file (.mmdb)
    #[arg(long)]
    pub geoip_db: Option<String>,

    /// Disable GeoIP and ASN enrichment
    #[arg(long)]
    pub no_enrich: bool,
}

#[derive(Parser, Debug, Clone)]
pub struct QuickArgs {
    /// Output file path
    #[arg(short, long, default_value = "proxies.txt")]
    pub output: String,

    /// Maximum number of proxies to check
    #[arg(long)]
    pub max: Option<usize>,

    /// Maximum concurrent checks
    #[arg(short, long, default_value_t = 512)]
    pub concurrency: usize,

    /// Limit output to top N fastest proxies
    #[arg(short, long)]
    pub limit: Option<usize>,
}
