use anyhow::Result;
use std::io::Write;

use crate::scanner::PortResult;

pub fn write(results: &[PortResult]) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", json)?;
    Ok(())
}
