use super::tcp;
use super::udp;
use super::{PortResult, PortState};
use crate::fingerprint::os_detect::detect_os;
use crate::fingerprint::services::identify_service;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

pub struct ScanEngine {
    concurrent: usize,
    timeout_ms: u64,
    rate_limit: u64,
    udp: bool,
    os_detect: bool,
}

impl ScanEngine {
    pub fn new(
        concurrent: usize,
        timeout_ms: u64,
        rate_limit: u64,
        udp: bool,
        os_detect: bool,
    ) -> Self {
        Self {
            concurrent,
            timeout_ms,
            rate_limit,
            udp,
            os_detect,
        }
    }

    pub async fn scan_all(&self, targets: &[String], ports: &[u16]) -> Vec<PortResult> {
        let semaphore = Arc::new(Semaphore::new(self.concurrent));
        let mut handles = Vec::new();
        let rate_delay = if self.rate_limit > 0 {
            Duration::from_secs_f64(1.0 / self.rate_limit as f64)
        } else {
            Duration::from_secs(0)
        };

        for target in targets {
            for &port in ports {
                let target = target.clone();
                let sem = Arc::clone(&semaphore);
                let timeout_ms = self.timeout_ms;
                let udp = self.udp;
                let os_detect = self.os_detect;
                let rate_delay = rate_delay;

                let handle = tokio::spawn(async move {
                    let _permit = sem.acquire().await.unwrap();

                    if rate_delay > Duration::from_secs(0) {
                        sleep(rate_delay).await;
                    }

                    let mut result = tcp::scan_tcp(&target, port, timeout_ms).await;

                    if result.state == PortState::Open {
                        if let Ok(Some(banner)) =
                            tcp::grab_banner(&target, port, timeout_ms).await
                        {
                            result.banner = Some(banner.clone());
                            result.service = Some(identify_service(port, Some(&banner)));
                        } else {
                            result.service = Some(identify_service(port, None));
                        }

                        if os_detect {
                            result.os_guess = detect_os(&target, port).await;
                        }
                    }

                    if udp && result.state != PortState::Open {
                        let udp_result = udp::scan_udp(&target, port, timeout_ms).await;
                        if udp_result.state == PortState::OpenFiltered {
                            result = udp_result;
                        }
                    }

                    result
                });

                handles.push(handle);
            }
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("Task failed: {}", e),
            }
        }

        results.sort_by(|a, b| {
            a.host
                .cmp(&b.host)
                .then(a.port.cmp(&b.port))
                .then(a.protocol.to_string().cmp(&b.protocol.to_string()))
        });

        results
    }
}
