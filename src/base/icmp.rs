use std::net::Ipv4Addr;
use std::str::FromStr;

use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::{
    icmp::{echo_request::MutableEchoRequestPacket, IcmpTypes},
    ip::IpNextHeaderProtocols,
    ipv4::{checksum as ip_checksum, Ipv4Flags, Ipv4Packet, MutableIpv4Packet},
    Packet
};
use pnet::util::{checksum as util_checksum, MacAddr};
use rand::Rng;



pub(crate) fn ping(host: &str) {
    

    let interfaces = datalink::interfaces();
    let interface = interfaces.get(1).unwrap();
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unable to create ethernet tunnel"),
        Err(e) => panic!("Unable to create channel: {e}")
    };

    'ping: for seq in 0..4 {

        // icmp packet
        let mut icmp_buff = [0u8; 8];
        let mut icmp_pkt = MutableEchoRequestPacket::new(&mut icmp_buff).unwrap();
        let identifier: u16 = rand::thread_rng().gen_range(0..1000);        
        {
            icmp_pkt.set_icmp_type(IcmpTypes::EchoRequest);
            icmp_pkt.set_identifier(identifier);
            icmp_pkt.set_sequence_number(seq + 1);
            icmp_pkt.set_checksum(util_checksum(icmp_pkt.packet(), 1));
        }

        // ip packet
        let mut ip_buf: Vec<u8> = vec![0u8; 20 + icmp_pkt.packet().len()];
        let total_len = ip_buf.len() as u16;
        let mut ip_pkt = MutableIpv4Packet::new(&mut ip_buf[..]).unwrap();
        let src = interface.ips.first().unwrap().ip();
        ip_pkt.set_version(4);
        ip_pkt.set_header_length(5);
        ip_pkt.set_dscp(4);
        ip_pkt.set_ecn(1);
        ip_pkt.set_total_length(total_len);
        ip_pkt.set_identification(2118);
        ip_pkt.set_flags(Ipv4Flags::DontFragment);
        ip_pkt.set_fragment_offset(0);
        ip_pkt.set_ttl(255);
        ip_pkt.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
        ip_pkt.set_source(Ipv4Addr::from_str(&src.to_string()).unwrap());
        ip_pkt.set_destination(Ipv4Addr::from_str(host).unwrap());
        ip_pkt.set_checksum(ip_checksum(&ip_pkt.to_immutable()));
        ip_pkt.set_payload(icmp_pkt.packet());
        
        // ethernet frame
        let mut eth_buf: Vec<u8> = vec![0u8; 14 + ip_pkt.packet().len()];
        let mut eth_pkt = MutableEthernetPacket::new(&mut eth_buf[..]).unwrap();
        eth_pkt.set_source(interface.mac.unwrap());
        eth_pkt.set_destination(MacAddr(0xd4, 0xb1, 0x08, 0x4c, 0xbb, 0xf9));
        eth_pkt.set_payload(ip_pkt.packet());
        eth_pkt.set_ethertype(EtherTypes::Ipv4);
        tx.send_to(eth_pkt.packet(), None);

        'receiving: loop {
            let incoming_buff = rx.next().unwrap();
            let incoming_eth_pkt = EthernetPacket::new(incoming_buff).unwrap();

            match incoming_eth_pkt.get_ethertype() {

                EtherTypes::Ipv4 => {
                    let incoming_ip = match Ipv4Packet::new(incoming_eth_pkt.payload()) {
                        Some(pkt) => pkt,
                        None => continue 'receiving
                    };
                    
                    match incoming_ip.get_next_level_protocol() {
                        IpNextHeaderProtocols::Icmp => {
                            match IcmpPacket::new(incoming_ip.payload()) {
                                Some(p) => {
                                    let echo = EchoReplyPacket::new(&p.packet()).unwrap();
                                    println!(
                                        "Successful: {} bytes from {:?}: icmp_seq={} ttl={}", 
                                        incoming_eth_pkt.packet().len(), 
                                        incoming_ip.get_source(), 
                                        echo.get_sequence_number(),
                                        incoming_ip.get_ttl() 
                                    );
                                    break 'receiving
                                },
                                None => {
                                    println!("corrupted icmp pkt received");
                                    continue 'ping
                                }
                            }
                        },
                        _ => continue 'receiving
                    };


                    
                }
                _ => continue 'receiving
            }
        } 
    }
    
}