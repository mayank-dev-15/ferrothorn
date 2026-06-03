# 🌿 Ferrothorn

**Blazing-fast async port scanner written in Rust.**

Ferrothorn is a high-performance network port scanner built with Tokio for async I/O. It supports TCP and UDP scanning, service fingerprinting via banner grabbing, and OS detection based on TCP/IP stack behavior.

## Features

- **Async TCP Port Scanning** — Powered by Tokio for maximum throughput
- **UDP Port Scanning** — Protocol-specific probes for DNS, NTP, SNMP, TFTP, NetBIOS, SSDP, mDNS
- **Service Fingerprinting** — Banner grabbing with protocol-aware probes
- **OS Detection** — TCP/IP stack behavior analysis (TTL, window size, banner heuristics)
- **Configurable Concurrency** — Control the number of simultaneous connections
- **Rate Limiting** — Token-bucket rate limiter to avoid overwhelming targets
- **Multiple Output Formats** — JSON, CSV, and colorful human-readable text
- **CIDR Support** — Scan entire subnets (e.g., `10.0.0.0/24`)

## Installation

```bash
git clone https://github.com/mayank-dev-15/ferrothorn.git
cd ferrothorn
cargo build --release
```

The binary will be at `./target/release/ferrothorn`.

## Usage

```
ferrothorn --target <IP or CIDR> --ports <range or list> [OPTIONS]
```

### Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--target` | `-t` | Target IP, CIDR range | *required* |
| `--ports` | `-p` | Ports: range (1-1024), list (22,80,443), single (80) | *required* |
| `--udp` | | Enable UDP scanning | false |
| `--os-detect` | | Enable OS detection | false |
| `--output` | `-o` | Output format: json, csv, text | text |
| `--concurrent` | `-c` | Max concurrent connections | 100 |
| `--timeout` | | Connection timeout in ms | 2000 |
| `--rate-limit` | | Rate limit (packets/sec, 0=unlimited) | 0 |

### Examples

```bash
# Scan common ports on a single host
ferrothorn --target 192.168.1.1 --ports 1-1024

# Scan specific ports with OS detection, output as JSON
ferrothorn --target 192.168.1.1 --ports 22,80,443 --os-detect --output json

# Scan a subnet with UDP and high concurrency
ferrothorn --target 10.0.0.0/24 --ports 22,80,443,8080 --udp --concurrent 500 --output csv

# Quick scan with rate limiting
ferrothorn --target 127.0.0.1 --ports 1-65535 --rate-limit 1000 --timeout 500
```

## Output Formats

### Text (default)
Colorful terminal output with scan summary, showing open/closed/filtered ports, service names, banners, and OS detection results.

### JSON
Machine-readable JSON output suitable for piping into `jq` or other tools:
```bash
ferrothorn --target 192.168.1.1 --ports 22,80,443 --output json | jq '.[] | select(.state=="Open")'
```

### CSV
Comma-separated output for spreadsheet import:
```bash
ferrothorn --target 192.168.1.1 --ports 1-1024 --output csv > results.csv
```

## Project Structure

```
ferrothorn/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, banner, output dispatch
│   ├── cli.rs               # CLI argument parsing with clap
│   ├── scanner/
│   │   ├── mod.rs           # Scanner module exports
│   │   ├── engine.rs        # Scan engine: concurrency, rate limiting, result collection
│   │   ├── tcp.rs           # Async TCP scanner with banner grabbing
│   │   └── udp.rs           # UDP scanner with protocol-specific probes
│   ├── fingerprint/
│   │   ├── mod.rs           # Fingerprint module exports
│   │   ├── services.rs      # Service fingerprinting from banners + well-known ports
│   │   └── os_detect.rs     # OS detection from TTL, window size, banners
│   ├── output/
│   │   ├── mod.rs           # Output module exports
│   │   ├── json.rs          # JSON output formatter
│   │   ├── csv.rs           # CSV output formatter
│   │   └── text.rs          # Colorful text output formatter
│   └── utils/
│       ├── mod.rs           # Utils module exports
│       └── network.rs       # Target expansion (CIDR, IP parsing)
├── README.md
└── .gitignore
```

## License

MIT
