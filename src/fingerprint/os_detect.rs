use std::net::IpAddr;
use std::str::FromStr;

/// Detect OS from a banner string (synchronous).
pub fn detect_os_from_banner(banner: &str) -> Option<String> {
    let lower = banner.to_lowercase();

    if lower.contains("ubuntu") {
        return Some("Linux (Ubuntu)".into());
    }
    if lower.contains("debian") {
        return Some("Linux (Debian)".into());
    }
    if lower.contains("centos") {
        return Some("Linux (CentOS)".into());
    }
    if lower.contains("fedora") {
        return Some("Linux (Fedora)".into());
    }
    if lower.contains("alpine") {
        return Some("Linux (Alpine)".into());
    }
    if lower.contains("arch") {
        return Some("Linux (Arch)".into());
    }
    if lower.contains("freebsd") {
        return Some("FreeBSD".into());
    }
    if lower.contains("openbsd") {
        return Some("OpenBSD".into());
    }
    if lower.contains("netbsd") {
        return Some("NetBSD".into());
    }
    if lower.contains("microsoft-iis") {
        return Some("Windows (IIS)".into());
    }
    if lower.contains("win32") || lower.contains("win64") {
        return Some("Windows".into());
    }
    if lower.starts_with("ssh-") && lower.contains("openssh") {
        return Some("Linux".into());
    }
    if lower.contains("darwin") || lower.contains("macos") || lower.contains("mac os") {
        return Some("macOS".into());
    }

    None
}

/// Async OS detection — tries banner first, then TCP/IP fingerprinting.
pub async fn detect_os(host: &str, port: u16) -> Option<String> {
    // Try a quick TCP connect and grab banner for OS detection
    let addr = format!("{}:{}", host, port);
    let Ok(Ok(mut stream)) = tokio::time::timeout(
        std::time::Duration::from_millis(2000),
        tokio::net::TcpStream::connect(&addr),
    )
    .await
    else {
        return None;
    };

    // Try to read a banner
    let mut buf = [0u8; 1024];
    if let Ok(Ok(n)) = tokio::time::timeout(
        std::time::Duration::from_millis(1500),
        stream.read(&mut buf),
    )
    .await
    {
        if n > 0 {
            let banner = String::from_utf8_lossy(&buf[..n]);
            if let Some(os) = detect_os_from_banner(&banner) {
                return Some(os);
            }
        }
    }

    // Try TTL-based detection from socket
    if let Ok(ttl) = stream.ttl() {
        return match ttl {
            0..=64 => Some("Linux/Unix".into()),
            65..=128 => Some("Windows".into()),
            129..=255 => Some("Cisco/Network Device".into()),
            _ => None,
        };
    }

    None
}

/// OS detection based on TCP/IP stack behavior.
pub struct OsDetector;

impl OsDetector {
    pub fn new() -> Self {
        OsDetector
    }

    pub fn guess_from_banner(&self, banner: &str) -> Option<String> {
        detect_os_from_banner(banner)
    }

    pub fn guess_from_ttl(&self, ttl: u8) -> Option<String> {
        match ttl {
            0..=64 => Some("Linux/Unix".into()),
            65..=128 => Some("Windows".into()),
            129..=255 => Some("Cisco/Network Device".into()),
            _ => None,
        }
    }

    pub fn guess_from_window_size(&self, window_size: u32) -> Option<String> {
        match window_size {
            5840 => Some("Linux (older kernel)".into()),
            5720 => Some("Linux".into()),
            65535 => Some("Windows".into()),
            65536 => Some("Linux (modern)".into()),
            16384 => Some("FreeBSD".into()),
            8192 => Some("macOS".into()),
            _ => None,
        }
    }
}

impl Default for OsDetector {
    fn default() -> Self {
        Self::new()
    }
}
