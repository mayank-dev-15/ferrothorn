```
     ╔══════════════════════════════════════════════════════╗
     ║  🦔  F E R R O T H O R N                            ║
     ║  Blazing-Fast Async Port Scanner                     ║
     ║  "Every port tells a story. We read them all."       ║
     ╚══════════════════════════════════════════════════════╝
```

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-0084C1?style=for-the-badge&logo=rust&logoColor=white)](https://tokio.rs/)
[![CLI](https://img.shields.io/badge/CLI-Tool-24292e?style=for-the-badge&logo=gnubash&logoColor=white)](https://en.wikipedia.org/wiki/Command-line_interface)
[![Network Security](https://img.shields.io/badge/Network-Security-e74c3c?style=for-the-badge&logo=protonvpn&logoColor=white)](https://en.wikipedia.org/wiki/Network_security)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

**A high-performance, asynchronous port scanner written in Rust — built for security professionals and network engineers.**

[Installation](#-installation) · [Usage](#-usage) · [Output Formats](#-output-formats) · [Architecture](#-architecture)

</div>

---

## What Is Ferrothorn?

Ferrothorn is a **multi-protocol port scanner** that leverages Rust's Tokio runtime for fully asynchronous I/O. It can scan thousands of ports across single hosts or entire subnets while maintaining a small memory footprint and predictable latency.

Named after the spiky Pokémon that punishes attackers on contact, Ferrothorn is designed to be the tool you reach for when you need to **map a network fast** — whether you're doing a CTF, auditing your own infrastructure, or learning how TCP/IP really works.

---

## ✨ Features

- **⚡ Async TCP Scanning** — Thousands of concurrent connections via Tokio's work-stealing runtime
- **📡 UDP Port Scanning** — Protocol-specific probes for DNS, NTP, SNMP, TFTP, NetBIOS, SSDP, mDNS
- **🔍 Service Fingerprinting** — Banner grabbing with protocol-aware probes (HTTP, SSH, FTP, SMTP, MySQL, Redis, and more)
- **🖥️ OS Detection** — TCP/IP stack fingerprinting via TTL analysis, TCP window size, and banner heuristics
- **📊 Multiple Output Formats** — JSON, CSV, and colorful human-readable terminal output
- **🌐 CIDR Support** — Scan entire subnets (e.g., `10.0.0.0/24`) with automatic target expansion
- **⏱️ Rate Limiting** — Token-bucket rate limiter to avoid overwhelming targets or triggering IDS
- **🔧 Configurable Concurrency** — Fine-tune the number of simultaneous connections
- **🎨 Colorful Terminal Output** — Rich formatting with scan summaries and progress indicators

---

## 📦 Installation

### From Source

```bash
git clone https://github.com/mayank-dev-15/ferrothorn.git
cd ferrothorn
cargo build --release
```

The optimized binary will be at `./target/release/ferrothorn`.

### Verify Installation

```bash
ferrothorn --version
```

### Requirements

- **Rust 1.75+** — Install via [rustup](https://rustup.rs/)
- **Linux / macOS / Windows** — All platforms supported

---

## 🚀 Usage

```
ferrothorn --target <IP or CIDR> --ports <range or list> [OPTIONS]
```

### Arguments

| Argument | Short | Description | Default |
|----------|-------|-------------|---------|
| `--target` | `-t` | Target IP address or CIDR range | *required* |
| `--ports` | `-p` | Ports: range `1-1024`, list `22,80,443`, single `80` | *required* |
| `--udp` | | Enable UDP scanning | `false` |
| `--os-detect` | | Enable OS detection via TCP/IP fingerprinting | `false` |
| `--output` | `-o` | Output format: `json`, `csv`, `text` | `text` |
| `--concurrent` | `-c` | Max concurrent connections | `100` |
| `--timeout` | | Connection timeout in milliseconds | `2000` |
| `--rate-limit` | | Max packets per second (0 = unlimited) | `0` |

### Examples

```bash
# Scan common ports on a single host
ferrothorn --target 192.168.1.1 --ports 1-1024

# Scan specific ports with OS detection, output as JSON
ferrothorn -t 192.168.1.1 -p 22,80,443 --os-detect -o json

# Full subnet scan with UDP and high concurrency
ferrothorn -t 10.0.0.0/24 -p 22,80,443,8080 --udp -c 500 -o csv

# Quick scan with rate limiting (stealthy)
ferrothorn -t 127.0.0.1 -p 1-65535 --rate-limit 1000 --timeout 500

# Pipe JSON output to jq for filtering
ferrothorn -t 192.168.1.1 -p 1-1024 -o json | jq '.[] | select(.state=="Open")'
```

---

## 📤 Output Formats

### Text (Default)

Rich terminal output with colors, scan summary, and per-port details:

```
╔══════════════════════════════════════════╗
║         FERROTHORN SCAN REPORT           ║
╠══════════════════════════════════════════╣
║  Target:   192.168.1.1                   ║
║  Ports:    1-1024                        ║
║  Duration: 3.2s                          ║
║  Open:     4                             ║
╚══════════════════════════════════════════╝

PORT    STATE   SERVICE     BANNER
22      Open    SSH         SSH-2.0-OpenSSH_8.9p1
80      Open    HTTP        Apache/2.4.54 (Ubuntu)
443     Open    HTTPS       —
8080    Open    HTTP-Proxy  nginx/1.22.1

OS Detection: Linux 5.x (confidence: 87%)
```

### JSON

Machine-readable output for automation and piping:

```json
[
  { "port": 22, "protocol": "tcp", "state": "Open", "service": "SSH", "banner": "SSH-2.0-OpenSSH_8.9p1" },
  { "port": 80, "protocol": "tcp", "state": "Open", "service": "HTTP", "banner": "Apache/2.4.54" }
]
```

### CSV

Spreadsheet-friendly output:

```csv
port,protocol,state,service,banner
22,tcp,Open,SSH,SSH-2.0-OpenSSH_8.9p1
80,tcp,Open,HTTP,Apache/2.4.54
```

---

## 🔍 Service Fingerprinting

Ferrothorn goes beyond simple port state detection. For every open TCP port, it:

1. **Sends a protocol-specific probe** (e.g., `HEAD / HTTP/1.0` for port 80, `EHLO` for port 25)
2. **Reads the banner** returned by the service
3. **Matches against a database** of known service signatures
4. **Reports the identified service** name and version

Supported protocols: HTTP, HTTPS, SSH, FTP, SMTP, POP3, IMAP, MySQL, PostgreSQL, Redis, MongoDB, DNS, Telnet, and more.

---

## 🖥️ OS Detection

When `--os-detect` is enabled, Ferrothorn analyzes:

| Signal | What It Reveals |
|--------|-----------------|
| **TTL (Time to Live)** | Initial TTL hints at OS family (64 → Linux/BSD, 128 → Windows, 255 → Cisco/Network gear) |
| **TCP Window Size** | Default window sizes are OS-specific |
| **TCP Options** | Option ordering and presence (MSS, SACK, timestamps, window scale) |
| **Service Banners** | SSH and HTTP banners often reveal the OS and version |

> ⚠️ **Disclaimer**: OS detection is heuristic-based and provides an educated guess, not a guarantee. Always verify manually.

---

## ⚡ Performance

Benchmarks on a typical machine (AMD Ryzen 5 / 16 Gbps network):

| Scan Type | Ports | Concurrency | Time |
|-----------|-------|-------------|------|
| Single host, common ports | 1,024 | 100 | ~2.1s |
| Single host, full range | 65,535 | 500 | ~18s |
| /24 subnet, top 100 ports | 25,600 | 1000 | ~12s |
| Full range, rate-limited | 65,535 | 100 | ~68s (at 1000 pps) |

Performance scales linearly with concurrency up to the network or target's capacity.

---

## 🏗️ Architecture

```
ferrothorn/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, banner, output dispatch
│   ├── cli.rs               # CLI argument parsing (clap)
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

**Design Principles:**
- **Async-first**: All I/O is non-blocking via Tokio
- **Modular**: Scanner, fingerprinting, and output are cleanly separated
- **Zero-copy**: Minimizes allocations in the hot path
- **Safe**: No `unsafe` code — Rust's ownership model guarantees memory safety

---

## ⚖️ Legal & Ethical Use

> **Only scan networks you own or have explicit written permission to test.** Unauthorized port scanning may violate computer fraud laws in your jurisdiction. The authors accept no responsibility for misuse of this tool.

---

## 📄 License

This project is open source under the [MIT License](https://opensource.org/licenses/MIT).

---

<div align="center">

🦔 *"Approach with caution."*

**⭐ Star this repo if Ferrothorn helped you map your network.**

</div>
