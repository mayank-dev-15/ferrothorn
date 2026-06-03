use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ferrothorn",
    version = "0.1.0",
    about = "A blazing-fast async port scanner written in Rust",
    long_about = "Ferrothorn — a high-performance asynchronous port scanner built with Rust and Tokio. \
                  Supports TCP and UDP scanning, service fingerprinting, OS detection, \
                  and multiple output formats."
)]
pub struct Args {
    /// Target IP address, CIDR range, or hostname
    #[arg(short, long, required = true)]
    pub target: String,

    /// Ports to scan: single (80), range (1-1024), or comma-separated list (22,80,443)
    #[arg(short, long, default_value = "1-1024")]
    pub ports: String,

    /// Enable UDP scanning (in addition to TCP)
    #[arg(long)]
    pub udp: bool,

    /// Attempt OS detection based on TTL and TCP window heuristics
    #[arg(long)]
    pub os_detect: bool,

    /// Output format: text, json, or csv
    #[arg(short, long, default_value = "text")]
    pub output: String,

    /// Maximum number of concurrent scan tasks
    #[arg(long, default_value_t = 100)]
    pub concurrent: usize,

    /// Connection timeout in milliseconds
    #[arg(long, default_value_t = 2000)]
    pub timeout: u64,

    /// Maximum packets per second (rate limit)
    #[arg(long, default_value_t = 1000)]
    pub rate_limit: u64,
}
