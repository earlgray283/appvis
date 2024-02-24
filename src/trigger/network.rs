use super::Trigger;
use anyhow::{bail, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command, time::Duration};
use tokio::{sync::mpsc::Sender, time::sleep};

#[derive(Debug, Serialize, Deserialize)]
pub struct WifiConnected {
    pub interval: Duration,
}

impl Trigger for WifiConnected {
    async fn observe(&self, tx: Sender<Result<()>>) {
        loop {
            let send_value = match is_wifi_connected() {
                Ok(true) => {
                    info!("WifiConnected triggered");
                    Ok(())
                }
                Err(e) => Err(e),
                _ => {
                    sleep(self.interval).await;
                    continue;
                }
            };
            if let Err(e) = tx.send(send_value).await {
                error!("failed to send result: {}", e);
            }
            return;
        }
    }

    fn channel_buffer_size(&self) -> usize {
        1
    }
}

fn is_wifi_connected() -> Result<bool> {
    let stdout = Command::new("ifconfig").output()?.stdout;
    let output = String::from_utf8_lossy(&stdout);

    let interface_mp = parse_ifconfig_output(&output);
    match interface_mp.get("en0") {
        Some(active) => Ok(*active),
        None => bail!("failed to find wifi network interface"),
    }
}

// network interface => active|inactive
fn parse_ifconfig_output(output: &str) -> HashMap<String, bool> {
    let mut mp = HashMap::new();
    let mut interface_name: Option<&str> = None;
    for line in output.lines() {
        if !line.starts_with('\t') {
            let (k, _) = line.split_once(": ").unwrap();
            interface_name = Some(k);
            continue;
        }
        let name = if let Some(name) = interface_name {
            name
        } else {
            continue;
        };
        match line.trim().split_once(": ") {
            Some(("status", v)) => {
                mp.insert(name.to_string(), v == "active");
                interface_name = None;
            }
            _ => continue,
        };
    }
    mp
}

#[cfg(test)]
mod test {
    use super::parse_ifconfig_output;

    #[test]
    fn test_parse_ifconfig_output() {
        let input = "ap1: flags=9999<UP,BROADCAST,RUNNING,SIMPLEX,MULTICAST> mtu 9999
\toptions=0000<TSO4,TSO6,CHANNEL_IO,PARTIAL_CSUM,ZEROINVERT_CSUM>
\tether 11:22:33:44:55:66
\tinet6 abcd::efe:1111:2222:cba2%ap1 prefixlen 64 scopeid 0xb 
\tnd6 options=201<PERFORMNUD,DAD>
\tmedia: autoselect (<unknown type>)
\tstatus: inactive
en0: flags=9999<UP,BROADCAST,SMART,RUNNING,SIMPLEX,MULTICAST> mtu 9999
\toptions=0000<TSO4,TSO6,CHANNEL_IO,PARTIAL_CSUM,ZEROINVERT_CSUM>
\tether 11:22:33:44:55:66
\tinet6 abcd::efe:1111:2222:3333%en0 prefixlen 64 secured scopeid 0xc 
\tinet6 0000:aaaa:1111:2222:3333:4444:5555:6666 prefixlen 64 autoconf secured 
\tinet6 0000:bbbb:2222:3333:4444:5555:6666:7777 prefixlen 64 autoconf temporary 
\tinet 0.0.0.0 netmask 0xffffffff broadcast 255.255.255.255
\tnd6 options=000<PERFORMNUD,DAD>
\tmedia: autoselect
\tstatus: active
";
        let mp = parse_ifconfig_output(input);
        assert_eq!(mp.get("ap1"), Some(&false));
        assert_eq!(mp.get("en0"), Some(&true));
        assert_eq!(mp.get("hoge"), None);
    }
}
