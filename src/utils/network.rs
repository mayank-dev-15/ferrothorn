use anyhow::{Context, Result};
use ipnetwork::IpNetwork;
use std::net::IpAddr;
use tokio::net::lookup_host;

/// Resolve a target string into a list of IP addresses.
pub async fn resolve_targets(target: &str) -> Result<Vec<String>> {
    if let Ok(network) = target.parse::<IpNetwork>() {
        return Ok(network.iter().map(|ip| ip.to_string()).collect());
    }

    if let Ok(ip) = target.parse::<IpAddr>() {
        return Ok(vec![ip.to_string()]);
    }

    let addrs = lookup_host(format!("{}:0", target))
        .await
        .with_context(|| format!("Failed to resolve hostname: {}", target))?;

    let ips: Vec<String> = addrs
        .filter_map(|addr| {
            let ip = addr.ip();
            if ip.is_ipv4() { Some(ip.to_string()) } else { None }
        })
        .collect();

    if ips.is_empty() {
        anyhow::bail!("No IPv4 addresses resolved for '{}'", target);
    }

    Ok(ips)
}

/// Parse a port specification string into a list of port numbers.
pub fn parse_ports(spec: &str) -> Result<Vec<u16>> {
    let mut ports = Vec::new();

    for part in spec.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                anyhow::bail!("Invalid port range: {}", part);
            }
            let start: u16 = range_parts[0].trim().parse()
                .with_context(|| format!("Invalid port number: {}", range_parts[0]))?;
            let end: u16 = range_parts[1].trim().parse()
                .with_context(|| format!("Invalid port number: {}", range_parts[1]))?;
            if start > end {
                anyhow::bail!("Invalid port range: {} > {}", start, end);
            }
            for p in start..=end {
                ports.push(p);
            }
        } else {
            let port: u16 = part.parse()
                .with_context(|| format!("Invalid port number: {}", part))?;
            ports.push(port);
        }
    }

    ports.sort_unstable();
    ports.dedup();

    for &p in &ports {
        if p == 0 {
            anyhow::bail!("Port 0 is not valid");
        }
    }

    Ok(ports)
}
