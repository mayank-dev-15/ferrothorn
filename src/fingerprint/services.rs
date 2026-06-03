/// Well-known port-to-service mappings.
pub struct WellKnownPorts;

impl WellKnownPorts {
    pub fn lookup(port: u16) -> Option<String> {
        match port {
            20 => Some("ftp-data".into()),
            21 => Some("ftp".into()),
            22 => Some("ssh".into()),
            23 => Some("telnet".into()),
            25 => Some("smtp".into()),
            53 => Some("dns".into()),
            67 => Some("dhcp-server".into()),
            68 => Some("dhcp-client".into()),
            69 => Some("tftp".into()),
            80 => Some("http".into()),
            110 => Some("pop3".into()),
            111 => Some("rpcbind".into()),
            119 => Some("nntp".into()),
            123 => Some("ntp".into()),
            135 => Some("msrpc".into()),
            137 => Some("netbios-ns".into()),
            138 => Some("netbios-dgm".into()),
            139 => Some("netbios-ssn".into()),
            143 => Some("imap".into()),
            161 => Some("snmp".into()),
            162 => Some("snmptrap".into()),
            179 => Some("bgp".into()),
            194 => Some("irc".into()),
            389 => Some("ldap".into()),
            443 => Some("https".into()),
            445 => Some("microsoft-ds".into()),
            465 => Some("smtps".into()),
            514 => Some("syslog".into()),
            515 => Some("printer".into()),
            520 => Some("rip".into()),
            523 => Some("ibm-db2".into()),
            554 => Some("rtsp".into()),
            587 => Some("smtp-submission".into()),
            631 => Some("ipp".into()),
            636 => Some("ldaps".into()),
            993 => Some("imaps".into()),
            995 => Some("pop3s".into()),
            1080 => Some("socks".into()),
            1433 => Some("mssql".into()),
            1434 => Some("mssql-monitor".into()),
            1521 => Some("oracle-db".into()),
            1723 => Some("pptp".into()),
            2049 => Some("nfs".into()),
            2082 => Some("cpanel".into()),
            2083 => Some("cpanel-ssl".into()),
            2181 => Some("zookeeper".into()),
            2375 => Some("docker".into()),
            2376 => Some("docker-ssl".into()),
            3000 => Some("grafana/dev".into()),
            3128 => Some("squid-proxy".into()),
            3306 => Some("mysql".into()),
            3389 => Some("rdp".into()),
            4443 => Some("pharos".into()),
            4567 => Some("tram".into()),
            5000 => Some("upnp/flask".into()),
            5060 => Some("sip".into()),
            5061 => Some("sip-tls".into()),
            5432 => Some("postgresql".into()),
            5555 => Some("freeciv".into()),
            5672 => Some("amqp".into()),
            5900 => Some("vnc".into()),
            6379 => Some("redis".into()),
            6443 => Some("kubernetes-api".into()),
            6660 => Some("irc".into()),
            6667 => Some("irc".into()),
            6697 => Some("irc-ssl".into()),
            7001 => Some("afs3-callback".into()),
            8000 => Some("http-alt".into()),
            8008 => Some("http-alt".into()),
            8080 => Some("http-proxy".into()),
            8086 => Some("influxdb".into()),
            8443 => Some("https-alt".into()),
            8888 => Some("http-alt".into()),
            9000 => Some("sonarqube/portainer".into()),
            9042 => Some("cassandra".into()),
            9090 => Some("prometheus".into()),
            9092 => Some("kafka".into()),
            9093 => Some("prometheus-alert".into()),
            9200 => Some("elasticsearch".into()),
            9300 => Some("elasticsearch-transport".into()),
            9418 => Some("git".into()),
            10000 => Some("snet-mgmt".into()),
            11211 => Some("memcached".into()),
            15672 => Some("rabbitmq-mgmt".into()),
            27017 => Some("mongodb".into()),
            27018 => Some("mongodb".into()),
            27019 => Some("mongodb".into()),
            50000 => Some("sap-mgmt".into()),
            _ => None,
        }
    }
}

/// Identify service from port and optional banner.
/// This is the function the engine calls.
pub fn identify_service(port: u16, banner: Option<&str>) -> String {
    // First try banner-based identification
    if let Some(banner) = banner {
        let lower = banner.to_lowercase();

        if lower.starts_with("ssh-") {
            if let Some(version) = extract_ssh_version(&lower) {
                return format!("ssh ({})", version);
            }
            return "ssh".into();
        }

        if lower.starts_with("http/") || lower.contains("server:") {
            if lower.contains("apache") {
                return "http (Apache)".into();
            }
            if lower.contains("nginx") {
                return "http (nginx)".into();
            }
            if lower.contains("microsoft-iis") {
                return "http (IIS)".into();
            }
            if lower.contains("express") {
                return "http (Express)".into();
            }
            return "http".into();
        }

        if lower.starts_with("220 ") && (lower.contains("ftp") || lower.contains("vsftpd") || lower.contains("proftpd") || lower.contains("pure-ftpd")) {
            if lower.contains("vsftpd") {
                return "ftp (vsftpd)".into();
            }
            if lower.contains("proftpd") {
                return "ftp (ProFTPD)".into();
            }
            if lower.contains("pure-ftpd") {
                return "ftp (Pure-FTPd)".into();
            }
            return "ftp".into();
        }

        if lower.starts_with("220 ") && (lower.contains("smtp") || lower.contains("esmtp") || lower.contains("postfix") || lower.contains("exim") || lower.contains("sendmail")) {
            if lower.contains("postfix") {
                return "smtp (Postfix)".into();
            }
            if lower.contains("exim") {
                return "smtp (Exim)".into();
            }
            if lower.contains("sendmail") {
                return "smtp (Sendmail)".into();
            }
            return "smtp".into();
        }

        if lower.contains("+ok") && lower.contains("pop") {
            return "pop3".into();
        }

        if lower.contains("* ok") && lower.contains("imap") {
            return "imap".into();
        }

        if lower.contains("mysql") {
            return "mysql".into();
        }

        if lower.contains("postgresql") {
            return "postgresql".into();
        }

        if lower.contains("redis") {
            return "redis".into();
        }

        if lower.contains("mongodb") {
            return "mongodb".into();
        }

        if lower.contains("elasticsearch") {
            return "elasticsearch".into();
        }

        if lower.contains("docker") {
            return "docker".into();
        }
    }

    // Fall back to well-known port lookup
    WellKnownPorts::lookup(port).unwrap_or_else(|| "unknown".into())
}

fn extract_ssh_version(banner: &str) -> Option<String> {
    let parts: Vec<&str> = banner.splitn(3, '-').collect();
    if parts.len() >= 3 {
        let software = parts[2].trim();
        let software = software.split_whitespace().next().unwrap_or(software);
        Some(software.to_string())
    } else if parts.len() >= 2 {
        Some(parts[1].trim().to_string())
    } else {
        None
    }
}
