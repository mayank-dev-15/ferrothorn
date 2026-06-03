mod cli;
mod scanner;
mod fingerprint;
mod output;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use output::OutputFormat;
use scanner::engine::ScanEngine;
use std::time::Instant;
use utils::network::resolve_targets;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let start = Instant::now();

    let targets = resolve_targets(&args.target).await?;
    if targets.is_empty() {
        eprintln!("No valid targets resolved from '{}'", args.target);
        std::process::exit(1);
    }

    let ports = utils::network::parse_ports(&args.ports)?;
    if ports.is_empty() {
        eprintln!("No valid ports specified");
        std::process::exit(1);
    }

    let format = match args.output.as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        _ => OutputFormat::Text,
    };

    // Print banner for text output
    if matches!(format, OutputFormat::Text) {
        println!("{}", output::text::BANNER);
        println!("{}", format!("  Target:  {}", args.target).bright_red());
        println!("  Ports:   {:?}", args.ports);
        if args.udp {
            println!("  Mode:    TCP + UDP");
        } else {
            println!("  Mode:    TCP");
        }
        if args.os_detect {
            println!("  OS Detection: enabled");
        }
        println!("  Concurrency:  {}", args.concurrent);
        println!("  Timeout:      {}ms", args.timeout);
        println!("  Rate Limit:   {} pps", args.rate_limit);
        println!("{}", "─".repeat(50).bright_black());
    }

    let engine = ScanEngine::new(
        args.concurrent,
        args.timeout,
        args.rate_limit,
        args.udp,
        args.os_detect,
    );

    let results = engine.scan_all(&targets, &ports).await;

    match format {
        OutputFormat::Json => output::json::write(&results)?,
        OutputFormat::Csv => output::csv::write(&results)?,
        OutputFormat::Text => output::text::write(&results),
    }

    Ok(())
}

use colored::Colorize;
