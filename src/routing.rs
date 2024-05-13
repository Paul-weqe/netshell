use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_DUMP, NLM_F_REQUEST};
use netlink_packet_route::{route::RouteMessage, RouteNetlinkMessage};
use netlink_sys::{protocols::NETLINK_ROUTE, Socket, SocketAddr};


/// to be used at a later time
pub fn _get_route_table() {

    let mut socket = Socket::new(NETLINK_ROUTE).unwrap();
    let _port_number = socket.bind_auto().unwrap().port_number();
    socket.connect(&SocketAddr::new(0, 0)).unwrap();

    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_REQUEST | NLM_F_DUMP;

    let mut pkt = NetlinkMessage::new(
        nl_hdr,
        NetlinkPayload::from(RouteNetlinkMessage::GetRoute(
            RouteMessage::default()
        ))
    );

    pkt.finalize();
    let mut buf = vec![0; pkt.header.length as usize];
    pkt.serialize(&mut buf[..]);

    socket.send(&buf[..], 0).unwrap();

    let mut recv_buffer = [0; 4096];
    let mut offset = 0;
    
    'outer: loop {
        let size = socket.recv(&mut &mut recv_buffer[..], 0).unwrap();
        
        loop {
            let bytes = &recv_buffer[offset..];
            let msg: NetlinkMessage<RouteNetlinkMessage> = 
                NetlinkMessage::deserialize(bytes).unwrap();

            match msg.payload {
                NetlinkPayload::Done(_) => break 'outer,
                NetlinkPayload::InnerMessage(
                    RouteNetlinkMessage::NewRoute(entry)
                ) => {
                    println!("{:#?}", entry);
                    println!("-----------");
                }
                NetlinkPayload::Error(err) => {
                    eprintln!("Received a netlink error message: {err:?}");
                    return;
                }
                _ => {}
            }

            offset += msg.header.length as usize;
            if offset == size || msg.header.length == 0 {
                offset = 0;
                break;
            }
        }
    }

}