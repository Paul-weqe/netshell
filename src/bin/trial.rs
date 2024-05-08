use libc::{ in6_rtmsg, nlmsghdr, setsockopt, socket, timeval, AF_NETLINK, NETLINK_ROUTE, NLM_F_DUMP, NLM_F_REQUEST, RTM_GETROUTE, SOCK_RAW, SOL_SOCKET, SO_RCVTIMEO};
use std::{mem, process};

const BUFFER_SIZE: usize = 4096;

fn main() {
    let sock = unsafe {
        socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE)
    };
    
    if sock < 1 { panic!("The netlink socket could not be created") }

    let msgbuf: [char; BUFFER_SIZE] = unsafe { mem::zeroed() };
    let buffer: [char; BUFFER_SIZE] = unsafe { mem::zeroed() };
    let mut nlmsg: nlmsghdr = unsafe { mem::zeroed() };
    let mut msgseq = 0;
    let tv: timeval;

    // Complete the nlmsg header 
    nlmsg.nlmsg_type = RTM_GETROUTE;
    nlmsg.nlmsg_flags = NLM_F_DUMP as u16 | NLM_F_REQUEST as u16;
    nlmsg.nlmsg_seq = msgseq + 1;
    nlmsg.nlmsg_pid = process::id();
    // nlmsg.nlmsg_len = fthis::NLMG_LENGTH(mem::size_of::<in6_rtmsg>());


    // unsafe {
    //     setsockopt(
    //         sock, 
    //         SOL_SOCKET, 
    //         SO_RCVTIMEO, 
    //         tv, 
    //         mem::size_of::<timeval>()
    //     );
    // };
    
}


mod fthis {
    use std::mem::size_of;

    use libc::{c_uint, nlmsghdr};

    const NLMSG_ALIGNTO : c_uint = 4;

    pub const fn NLMSG_ALIGN(len: c_uint) -> c_uint {
        (len + NLMSG_ALIGNTO - 1) & !(NLMSG_ALIGNTO - 1)
    }

    pub const fn NLMG_LENGTH(len: c_uint) -> c_uint {
        len + NLMSG_ALIGN(size_of::<nlmsghdr>() as c_uint)
    }
}