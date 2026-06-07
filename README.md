# proxyhunt

A fast, clean, and powerful CLI-only proxy scraper and checker written in Rust.

`proxyhunt` is designed for speed and ease of use. It scrapes proxies from multiple sources, checks them concurrently, and outputs the fastest ones.

## Features

- **Smart Parsing**: Supports `IP:Port`, `user:pass@IP:Port`, `protocol://IP:Port`, and more.
- **High Performance**: Async checking using `tokio` and `reqwest` with configurable concurrency.
- **Multi-Protocol**: Supports HTTP, SOCKS4, and SOCKS5.
- **Flexible Sources**: Scrape from URLs or local text files.
- **Top-N Output**: Automatically sorts by latency and can limit output to the top $N$ fastest proxies.
- **Configurable**: Use CLI flags or a `config.toml` file.

## Installation

```bash
cargo install --path .
```

## Usage

### Basic Check
Check HTTP and SOCKS5 proxies from default sources:
```bash
proxyhunt check --http --socks5 --sources https://example.com/proxies.txt
```

### High Concurrency
```bash
proxyhunt check --http --concurrency 1024 --output my_fast_proxies.txt
```

### Using a Config File
```bash
proxyhunt --config my_config.toml check
```

## CLI Options

```text
Usage: proxyhunt [OPTIONS] <COMMAND>

Commands:
  check  Scrape and check proxies
  help   Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>    [default: config.toml]
  -v, --verbose            
  -h, --help               Print help
```

### `check` Subcommand Options

- `--http`, `--socks4`, `--socks5`: Enable specific protocols.
- `--sources <URL_OR_FILE>`: Add a source (can be used multiple times).
- `--output <FILE>`: Path to save working proxies (default: `proxies.txt`).
- `--json <FILE>`: Path to save working proxies in JSON format.
- `--limit <NUMBER>`: Only save the top $N$ fastest proxies.
- `--concurrency <NUMBER>`: Number of concurrent checks (default: 512).
- `--timeout <SECONDS>`: Total timeout per check (default: 10).
- `--connect-timeout <SECONDS>`: Connection timeout per check (default: 5).
- `--check-url <URL>`: URL used to verify proxy (default: `https://ipv4.icanhazip.com`).
- `--no-enrich`: Disable GeoIP enrichment.

## License

MIT
