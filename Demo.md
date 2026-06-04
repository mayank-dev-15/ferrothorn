# ferrothorn — Demo Documentation

## 🚀 Live Demo

**Try it now:** [https://mayank-dev-15.github.io/ferrothorn-demo](https://mayank-dev-15.github.io/ferrothorn-demo)

> **No installation required for HTML demos.** Runs entirely in your browser — no frameworks, no build tools, no backend. Pure vanilla HTML, CSS, and JavaScript.

---

## About

**ferrothorn** is a blazing-fast async port scanner.

### Key Features

- Async TCP connect scanner (tokio)
- UDP scanning with protocol probes
- Service fingerprinting (HTTP, SSH, FTP, SMTP)
- OS detection via TTL/window analysis
- JSON, CSV, Text output formats
- Configurable concurrency and rate limiting
- CIDR range scanning
- Colored terminal output

---

## Tech Stack

- Rust
- Tokio
- clap
- serde
- colored
- anyhow

---

## Controls & Usage

| Control | Action |
|---------|--------|
| `--target` | Set target IP/CIDR |
| `--ports` | Port range/list |
| `--udp` | Enable UDP scan |
| `--os-detect` | OS fingerprinting |
| `--output` | Format: json/csv/text |
| `--concurrent` | Thread count |
| `--timeout` | Timeout ms |

---

## Demo Screenshots

> Add screenshots by placing `screenshot.png` in the repo root.

---

## How to Run Locally

```bash
git clone https://github.com/mayank-dev-15/ferrothorn.git
cd ferrothorn
```

See the [README](https://github.com/mayank-dev-15/ferrothorn#readme) for detailed setup instructions.

---

## 🔗 Links

- **Live Demo:** [https://mayank-dev-15.github.io/ferrothorn-demo](https://mayank-dev-15.github.io/ferrothorn-demo)
- **Source:** [github.com/mayank-dev-15/ferrothorn](https://github.com/mayank-dev-15/ferrothorn)
- **Issues:** [github.com/mayank-dev-15/ferrothorn/issues](https://github.com/mayank-dev-15/ferrothorn/issues)
- **Releases:** [github.com/mayank-dev-15/ferrothorn/releases](https://github.com/mayank-dev-15/ferrothorn/releases)

---

*Built by [Mayank Basena](https://github.com/mayank-dev-15) · 15 · GSoC 2027 Aspirant*
