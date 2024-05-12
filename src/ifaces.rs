use pnet::datalink::NetworkInterface;
use tabled::{builder::Builder, settings::{object::Rows, Alignment, Style}};

use crate::config::InterfaceConfig;


pub fn draw_interface(ifaces: Vec<NetworkInterface>, brief: bool) {

    if brief {

        let mut builder = Builder::default();
        let header = vec![
            "name", "ip", "mac"
        ];

        builder.push_record(header);

        for interface in ifaces {
            let mut row: Vec<&str> = vec![];
            let mac = match interface.mac {
                Some(m) => {
                    m.to_string()
                }
                None => String::default()
            };
            let ip = match interface.ips.first() {
                Some(i) => {
                    i.to_string()
                },
                None => String::default()  
            };
            row.push(&interface.name);
            row.push(&ip);
            row.push(&mac);
            builder.push_record(row);
        }

        let table = builder.build()
            .with(Style::rounded())
            .modify(Rows::new(1..), Alignment::left())
            .to_string();

        println!("\n{table}");
    } else {
        
        let mut converted_ifaces = vec![];
        for iface in ifaces {
            converted_ifaces.push(InterfaceConfig::from(iface));
        }
        println!();
        let x =  serde_json::to_string_pretty(&converted_ifaces).unwrap();
        print!("{x}");
    }

}
