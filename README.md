# proxyhunt

[![Crates.io](https://img.shields.io/crates/v/proxyhunt.svg)](https://crates.io/crates/proxyhunt)
[![Documentation](https://docs.rs/proxyhunt/badge.svg)](https://docs.rs/proxyhunt)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/mithun/proxyhunt/workflows/CI/badge.svg)](https://github.com/mithun/proxyhunt/actions)

A high-performance, asynchronous proxy scraper and checker written in Rust.

`proxyhunt` is a streamlined command-line utility designed for speed and reliability. It efficiently fetches proxy lists from multiple remote or local sources, validates them concurrently using the Tokio runtime, and provides sorted output based on connection latency.

## Features

- **Asynchronous Architecture**: Leverages Tokio and Reqwest for massive concurrency.
- **Robust Parsing**: Advanced regex-based parsing supporting various formats:
  - `ip:port`
  - `user:pass@ip:port`
  - `protocol://ip:port`
- **Multi-Protocol Support**: Full validation for HTTP, SOCKS4, and SOCKS5.
- **Deduplication**: Automatically handles redundant proxies across multiple sources.
- **Flexible Configuration**: Seamless integration between CLI arguments and TOML configuration files.
- **Sorted Output**: Verified proxies are sorted by latency (fastest first).
- **Extensible Export**: Supports both plain-text (standard list) and detailed JSON reporting.

## Installation

### Via Cargo (Recommended)

Install `proxyhunt` directly from crates.io:

```bash
cargo install proxyhunt
```

### From Source

```bash
cargo install --path .
```

## Quick Start

### Quick Scrape & Check
Immediately scrape high-quality built-in sources and check them:

```bash
proxyhunt quick --max 1000 --limit 50
```

### Manual Check
Check HTTP proxies from a specific URL:

```bash
proxyhunt check --http --sources https://raw.githubusercontent.com/user/repo/main/proxies.txt
```

### Advanced Scrutiny

Run a high-concurrency check with custom timeouts and output to JSON:

```bash
proxyhunt check --http --socks5 --concurrency 1024 --timeout 5 --json verified.json
```

### Using Configuration

Generate a `config.toml` based on the provided example and run:

```bash
proxyhunt --config config.toml check
```

## Configuration

`proxyhunt` looks for `config.toml` by default. Key parameters include:

- `sources`: List of URLs or file paths.
- `concurrency`: Maximum simultaneous connection attempts.
- `limit`: Number of top-performing proxies to retain.
- `check_url`: The endpoint used for validation (defaults to icanhazip).

## CLI Reference

### Global Options

| Flag | Description | Default |
|------|-------------|---------|
| `--config`, `-c` | Path to the configuration file | `config.toml` |
| `--verbose`, `-v` | Enable debug-level logging | `false` |

### Check Subcommand

| Argument | Description | Default |
|----------|-------------|---------|
| `--http` | Enable HTTP proxy checking | `false` |
| `--socks4` | Enable SOCKS4 proxy checking | `false` |
| `--socks5` | Enable SOCKS5 proxy checking | `false` |
| `--sources`, `-s` | URLs or files to scrape (repeatable) | `[]` |
| `--output`, `-o` | Output file for verified proxies | `proxies.txt` |
| `--json` | Optional JSON output path | `None` |
| `--max` | Maximum number of proxies to check | `None` |
| `--concurrency` | Max parallel checks | `512` |
| `--timeout` | Total request timeout in seconds | `10` |
| `--limit`, `-l` | Limit output to top N results | `None` |

### Quick Subcommand

| Argument | Description | Default |
|----------|-------------|---------|
| `--output`, `-o` | Output file for verified proxies | `proxies.txt` |
| `--max` | Maximum number of proxies to check | `None` |
| `--concurrency` | Max parallel checks | `512` |
| `--limit`, `-l` | Limit output to top N results | `None` |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
