
pub(crate) mod icmp;
use std::{ffi::CString, mem::zeroed, str};

use libc::{self, utsname};

pub(crate) fn sethostname(hostname: &str) -> i32 {
    let size = hostname.len();
    let hostname = match CString::new(hostname) {
        Ok(h) => h,
        Err(_) => return -1
    };

    unsafe {
        libc::sethostname(CString::into_raw(hostname), size)
    }
}

pub(crate) fn get_name_structure() -> utsname {
    let mut result: utsname = unsafe { zeroed() };
    unsafe { libc::uname(&mut result) };
    let x = result.nodename;
    result
    // let mut buff: Vec<u8> = Vec::new();
    // buf.nodename.iter().for_each(|s| buff.push(*s as u8));
    // String::from( ext_str::from_utf8(&buff).unwrap() )
}

pub(crate) fn get_hostname() -> String {
    char_array_to_string(get_name_structure().nodename)
}

fn char_array_to_string(x: [i8; 65]) -> String {
    let mut buf: Vec<u8> = Vec::new();
    x.iter().for_each(|x| buf.push(*x as u8));
    String::from(str::from_utf8(&buf).unwrap())
}
