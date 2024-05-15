use std::mem::zeroed;
use std::str as s;

use libc::{utsname};

fn main() {
    let mut buf: utsname= unsafe {zeroed()} ;
    let answer = unsafe {
        // gethostname(*mut buf, len)
        libc::uname(&mut buf)
    };
    println!("{answer}");
    let mut new_buff: Vec<u8> = Vec::new();
    buf.nodename.iter().for_each(|a| new_buff.push(*a as u8));
    let name = s::from_utf8(&new_buff).unwrap().trim_matches(char::from(0));
    println!("{:?}", name);
}