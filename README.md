# ferrothorn

[![Live Demo](https://img.shields.io/badge/🚀_Live_Demo-Visit-blue?style=for-the-badge)](https://mayank-dev-15.github.io/ferrothorn-demo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Language](https://img.shields.io/badge/Language-Rust-green)

Blazing-fast async port scanner with TCP/UDP scanning, service fingerprinting, OS detection, and multi-format output.

`security` `port-scanner` `rust` `network` `cli`

---

## ✨ Features

- Async TCP connect scanner using tokio
- UDP scanning with protocol-specific probes (DNS, NTP, SNMP)
- Service fingerprinting via banner grabbing (HTTP, SSH, FTP, SMTP)
- OS detection based on TTL and TCP window size heuristics
- JSON, CSV, and colored text output formats
- Configurable concurrency with semaphore-based rate limiting
- CIDR range scanning support
- Colored terminal output with progress indicators

---

## 🚀 Live Demo

**[View Demo →](https://mayank-dev-15.github.io/ferrothorn-demo)**

The demo is hosted on GitHub Pages. No installation needed — just click and explore.

---

## 🛠️ Tech Stack

- Rust
- Tokio (async runtime)
- clap (CLI parsing)
- serde (serialization)
- colored (terminal colors)
- anyhow (error handling)

---

## 📦 Installation

```bash
git clone https://github.com/mayank-dev-15/ferrothorn.git
cd ferrothorn
```

```bash
cd ferrothorn
cargo build --release
# Binary at: ./target/release/ferrothorn
```

---

## 💡 Usage

```bash
# Basic scan
ferrothorn --target 192.168.1.1 --ports 1-1024

# Full scan with OS detection
ferrothorn --target 10.0.0.1 --ports 1-65535 --udp --os-detect --output json

# Fast scan of common ports
ferrothorn --target example.com --ports 22,80,443,8080 --concurrent 500

# Export results
ferrothorn --target 192.168.1.0/24 --ports 1-1024 --output csv > results.csv
```

---

## 📁 Project Structure

```
ferrothorn/
├── README.md          # This file
├── Demo.md            # Demo documentation
├── LICENSE            # MIT License
└── ...                # Source files
```

---

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

---

## 📄 License

This project is licensed under the MIT License.

---

## 🔗 Links

- **Live Demo:** [https://mayank-dev-15.github.io/ferrothorn-demo](https://mayank-dev-15.github.io/ferrothorn-demo)
- **Source Code:** [github.com/mayank-dev-15/ferrothorn](https://github.com/mayank-dev-15/ferrothorn)
- **Issues:** [github.com/mayank-dev-15/ferrothorn/issues](https://github.com/mayank-dev-15/ferrothorn/issues)
- **Releases:** [github.com/mayank-dev-15/ferrothorn/releases](https://github.com/mayank-dev-15/ferrothorn/releases)
- **Demo Docs:** [Demo.md](https://github.com/mayank-dev-15/ferrothorn/blob/main/Demo.md)

---

*Built with ❤️ by [Mayank Basena](https://github.com/mayank-dev-15) · 15 · GSoC 2027 Aspirant*
