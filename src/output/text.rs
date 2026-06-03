use colored::Colorize;

use crate::scanner::{PortResult, PortState};

pub fn write(results: &[PortResult]) {
    if results.is_empty() {
        println!("{}", "No results found.".yellow());
        return;
    }

    println!(
        "\n{:<20} {:<8} {:<8} {:<12} {:<16} {:<30} {}",
        "HOST".bold().underline(),
        "PORT".bold().underline(),
        "PROTO".bold().underline(),
        "STATE".bold().underline(),
        "SERVICE".bold().underline(),
        "BANNER".bold().underline(),
        "OS GUESS".bold().underline()
    );

    let mut open_count = 0;
    let mut filtered_count = 0;
    let mut closed_count = 0;

    for r in results {
        let state_str = match r.state {
            PortState::Open => {
                open_count += 1;
                "OPEN".green().bold()
            }
            PortState::Closed => {
                closed_count += 1;
                "closed".red()
            }
            PortState::Filtered => {
                filtered_count += 1;
                "filtered".yellow()
            }
            PortState::OpenFiltered => {
                open_count += 1;
                "open|filtered".cyan()
            }
        };

        let service = r.service.as_deref().unwrap_or("-");
        let banner = r.banner.as_deref().unwrap_or("-");
        let banner_display = if banner.len() > 28 {
            format!("{}..", &banner[..28])
        } else {
            banner.to_string()
        };
        let os = r.os_guess.as_deref().unwrap_or("-");

        println!(
            "{:<20} {:<8} {:<8} {:<12} {:<16} {:<30} {}",
            r.host.white(),
            r.port.to_string().cyan(),
            r.protocol.to_string().dimmed(),
            state_str,
            service.magenta(),
            banner_display.dimmed(),
            os.blue()
        );
    }

    println!(
        "\n{} {} open, {} filtered, {} closed — {} total",
        "▸".green().bold(),
        open_count.to_string().green().bold(),
        filtered_count.to_string().yellow(),
        closed_count.to_string().red(),
        results.len().to_string().bold()
    );
}
