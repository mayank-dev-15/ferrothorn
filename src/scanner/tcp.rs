use super::{PortResult, PortState, Protocol};
use anyhow::Result;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub async fn scan_tcp(host: &str, port: u16, timeout_ms: u64) -> PortResult {
    let addr = format!("{}:{}", host, port);
    let dur = Duration::from_millis(timeout_ms);

    match timeout(dur, TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => PortResult {
            host: host.to_string(),
            port,
            protocol: Protocol::Tcp,
            state: PortState::Open,
            service: None,
            banner: None,
            os_guess: None,
        },
        Ok(Err(_)) => PortResult {
            host: host.to_string(),
            port,
            protocol: Protocol::Tcp,
            state: PortState::Closed,
            service: None,
            banner: None,
            os_guess: None,
        },
        Err(_) => PortResult {
            host: host.to_string(),
            port,
            protocol: Protocol::Tcp,
            state: PortState::Filtered,
            service: None,
            banner: None,
            os_guess: None,
        },
    }
}

pub async fn grab_banner(host: &str, port: u16, timeout_ms: u64) -> Result<Option<String>> {
    let addr = format!("{}:{}", host, port);
    let dur = Duration::from_millis(timeout_ms);

    let connect_result = timeout(dur, TcpStream::connect(&addr)).await;
    let mut stream = match connect_result {
        Ok(Ok(s)) => s,
        _ => return Ok(None),
    };

    let probe = match port {
        80 | 8080 | 8443 => Some(b"HEAD / HTTP/1.0\r\n\r\n".to_vec()),
        21 => Some(b"USER anonymous\r\n".to_vec()),
        25 | 587 => Some(b"EHLO ferrothorn\r\n".to_vec()),
        _ => None,
    };

    if let Some(data) = probe {
        let _ = timeout(Duration::from_millis(500), stream.write_all(&data)).await;
    }

    let mut buf = [0u8; 1024];
    match timeout(Duration::from_millis(1000), stream.read(&mut buf)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buf[..n])
                .trim()
                .replace('\r', " ")
                .replace('\n', " ");
            let banner = banner.trim().to_string();
            if banner.is_empty() {
                Ok(None)
            } else {
                Ok(Some(banner))
            }
        }
        _ => Ok(None),
    }
}
