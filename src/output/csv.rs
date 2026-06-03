use anyhow::Result;
use std::io::Write;

use crate::scanner::{PortResult, PortState, Protocol};

pub fn write(results: &[PortResult]) -> Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "host,port,protocol,state,service,banner,os_guess")?;

    for r in results {
        let banner = r.banner.as_deref().unwrap_or("").replace('"', "\"\"");
        let service = r.service.as_deref().unwrap_or("");
        let os = r.os_guess.as_deref().unwrap_or("");
        let proto = match r.protocol {
            Protocol::Tcp => "tcp",
            Protocol::Udp => "udp",
        };
        let state = match r.state {
            PortState::Open => "open",
            PortState::Closed => "closed",
            PortState::Filtered => "filtered",
            PortState::OpenFiltered => "open|filtered",
        };
        writeln!(
            handle,
            "{},{},{},{},{},\"{}\",\"{}\"",
            r.host, r.port, proto, state, service, banner, os
        )?;
    }

    Ok(())
}
