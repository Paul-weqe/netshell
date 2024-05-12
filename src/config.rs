// module meant for getting the configurations of items inside the networking device
// examples are: interfaces, routing protocols etc

use pnet::{datalink::NetworkInterface, util::MacAddr};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub(crate) struct InterfaceConfig {
    name: String,
    description: String,
    index: u32,
    mac: String
}


impl From<NetworkInterface> for InterfaceConfig {
    fn from(iface: NetworkInterface) -> Self {
        Self {
            name: iface.name,
            description: iface.description,
            index: iface.index,
            mac: match iface.mac {
                Some(m) => mac_string(m),
                None => String::new()
            }
        }
    }
}

fn mac_string(mac: MacAddr) -> String {
    return format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac.0, mac.1, mac.2, mac.3, mac.4, mac.5
    )
}