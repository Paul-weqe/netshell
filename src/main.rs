// use std::{intrinsics::size_of, mem::size_of, os::unix::net::SocketAddr};

use std::mem::{self, size_of};

use libc::{self, bind, c_void, iovec, msghdr, nlmsghdr, send, sockaddr_nl, AF_INET, AF_NETLINK, NETLINK_ROUTE, NLM_F_DUMP, NLM_F_REQUEST, RTM_GETROUTE, SOCK_RAW};

type UChar = libc::c_uchar; 
type UInt = libc::c_uint;


fn main() {
    let sock = open_netlink();
    if dump_route_request(sock) < 0 {
        panic!("Failed to perform request")
    }

}

fn get_route_dump_response(sock: i32) {
    let mut nladdr: sockaddr_nl;
    let mut lov: iovec;
    let mut msg: msghdr;
}

fn dump_route_request(sock: i32) -> isize {
    #[repr(C)]
    struct NlRequest {
        nlh: nlmsghdr,
        rtm: rtmsg
    }

    let mut nl_request: NlRequest = unsafe { mem::zeroed() };
    nl_request.nlh.nlmsg_type = RTM_GETROUTE;
    nl_request.nlh.nlmsg_flags = (NLM_F_REQUEST | NLM_F_DUMP) as u16;
    nl_request.nlh.nlmsg_len = size_of::<NlRequest>() as u32;
    nl_request.rtm.rtm_family = AF_INET as u8;
    
    unsafe {
        send(
            sock, 
            any_as_u8_slice(&nl_request).as_ptr() as *const c_void,
            size_of::<NlRequest>(),
            0
        )
    }

}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}


fn open_netlink() -> i32 {
    let mut saddr: libc::sockaddr_nl = unsafe {
        std::mem::zeroed()
    };
    
    let fd = unsafe{
        libc::socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE) 
    };
    if fd < 0 { panic!("Failed to open netlink socket") }
    

    saddr.nl_family = libc::AF_NETLINK as u16;
    saddr.nl_pid = std::process::id();
    
    let saddr = (&saddr as *const libc::sockaddr_nl) as *const libc::sockaddr;
    let binding = unsafe {
        bind(
            fd, 
            saddr,
            size_of::<libc::sockaddr_nl>() as libc::socklen_t
        )
    };

    if binding < 0 {
        panic!("Unable to bind to netlink socket")
    }
    fd
}


#[repr(C)]
struct rtmsg {
    
    rtm_family: UChar,    
    // unsigned char rtm_family;   /* Address family of route */

    rtm_dst_len: UChar,
    // unsigned char rtm_dst_len;  /* Length of destination */
    
    rtm_src_len: UChar,
    // unsigned char rtm_src_len;  /* Length of source */,

    rtm_tos: UChar,
    // unsigned char rtm_tos;      /* TOS filter */

    rtm_table: UChar,
    // unsigned char rtm_table;    /* Routing table ID;
    //                                see RTA_TABLE below */

    rtm_protocol: UChar,
    // unsigned char rtm_protocol; /* Routing protocol; see below */

    rtm_scope: UChar,
    // unsigned char rtm_scope;    /* See below */

    rtm_type: UChar,
    // unsigned char rtm_type;     /* See below */

    rtm_flags: UInt
    // unsigned int  rtm_flags;
}
