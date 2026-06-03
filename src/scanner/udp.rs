use super::{PortResult, PortState, Protocol};
use rand::Rng;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

fn udp_probe_payload(port: u16) -> Vec<u8> {
    match port {
        53 => vec![
            0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x07, b'e', b'x', b'a',
            b'm', b'p', b'l', b'e', 0x03, b'c', b'o', b'm',
            0x00, 0x00, 0x01, 0x00, 0x01,
        ],
        123 => {
            let mut pkt = vec![0u8; 48];
            pkt[0] = 0x1b;
            let mut rng = rand::thread_rng();
            rng.fill(&mut pkt[12..]);
            pkt
        }
        161 => vec![
            0x30, 0x26, 0x02, 0x01, 0x00, 0x04, 0x06,
            b'p', b'u', b'b', b'l', b'i', b'c',
            0xa0, 0x19, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00,
            0x02, 0x01, 0x00, 0x30, 0x0b, 0x30, 0x09,
            0x06, 0x05, 0x2b, 0x06, 0x01, 0x02, 0x01,
            0x05, 0x00,
        ],
        _ => {
            let mut rng = rand::thread_rng();
            let mut p = vec![0u8; 4];
            rng.fill(&mut p[..]);
            p
        }
    }
}

pub async fn scan_udp(host: &str, port: u16, timeout_ms: u64) -> PortResult {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .unwrap_or_else(|_| "0.0.0.0:0".parse().unwrap());

    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => {
            return PortResult {
                host: host.to_string(),
                port,
                protocol: Protocol::Udp,
                state: PortState::Filtered,
                service: None,
                banner: None,
                os_guess: None,
            }
        }
    };

    let payload = udp_probe_payload(port);
    let dur = Duration::from_millis(timeout_ms);

    if socket.send_to(&payload, addr).await.is_err() {
        return PortResult {
            host: host.to_string(),
            port,
            protocol: Protocol::Udp,
            state: PortState::Filtered,
            service: None,
            banner: None,
            os_guess: None,
        };
    }

    let mut buf = [0u8; 2048];
    match timeout(dur, socket.recv_from(&mut buf)).await {
        Ok(Ok((n, _))) => {
            let response = String::from_utf8_lossy(&buf[..n])
                .trim()
                .replace('\r', " ")
                .replace('\n', " ");
            let banner = if response.is_empty() { None } else { Some(response) };
            PortResult {
                host: host.to_string(),
                port,
                protocol: Protocol::Udp,
                state: PortState::OpenFiltered,
                service: None,
                banner,
                os_guess: None,
            }
        }
        _ => PortResult {
            host: host.to_string(),
            port,
            protocol: Protocol::Udp,
            state: PortState::OpenFiltered,
            service: None,
            banner: None,
            os_guess: None,
        },
    }
}
