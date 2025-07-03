use get_if_addrs::get_if_addrs;
use sysinfo::{System, Networks};
use dns_lookup::lookup_host;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addr: IpAddr,
    pub mac_addr: String,
    pub received: u64,
    pub transmitted: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DnsInfo {
    pub hostname: String,
    pub ips: Vec<IpAddr>,
}

#[derive(Debug)]
pub struct NetworkData {
    pub interfaces: Vec<NetworkInterface>,
    pub dns_info: Option<DnsInfo>,
    system: System,
    networks: Networks,
}

impl NetworkData {
    pub fn new() -> Self {
        NetworkData {
            interfaces: Vec::new(),
            dns_info: None,
            system: System::new(),
            networks: Networks::new(),
        }
    }

    pub fn refresh(&mut self) {
        self.networks.refresh(true);

        self.interfaces = get_if_addrs().unwrap_or_default().into_iter().filter_map(|iface| {
            if iface.is_loopback() { return None; }

            let stats = self.networks.iter().find(|(name, _)| *name == &iface.name);

            let (received, transmitted) = if let Some((_, data)) = stats {
                (data.received(), data.transmitted())
            } else {
                (0, 0)
            };

            Some(NetworkInterface {
                name: iface.name,
                ip_addr: iface.addr.ip(),
                mac_addr: "N/A".to_string(),
                received,
                transmitted,
            })
        }).collect();

        if self.dns_info.is_none() {
            if let Ok(hostname) = dns_lookup::get_hostname() {
                let ips = lookup_host(&hostname).unwrap_or_default();
                self.dns_info = Some(DnsInfo {
                    hostname,
                    ips,
                });
            }
        }
    }
}