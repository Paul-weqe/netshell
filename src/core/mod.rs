
pub mod auth;
pub(crate) mod icmp;
pub(crate) mod history;
use std::{ffi::CString, mem::zeroed, str as ex_str};

use libc;

/// meant to hold the utsname structure as specified in posix
/// <https://pubs.opengroup.org/onlinepubs/7908799/xsh/sysutsname.h.html> 
pub struct NameStructure {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
    pub domainname: [u8; 65] 
}

impl From<libc::utsname> for NameStructure {
    fn from(value: libc::utsname) -> Self {
        let mut sysname: [u8; 65] = [u8::default(); 65];
        value.sysname.iter().enumerate().for_each(|(i, val)|sysname[i] = *val as u8);

        let mut nodename: [u8; 65] = [u8::default(); 65];
        value.nodename.iter().enumerate().for_each(|(i, val)| nodename[i] = *val as u8);

        let mut release: [u8; 65] = [u8::default(); 65];
        value.release.iter().enumerate().for_each(|(i, val)| release[i] = *val as u8);

        let mut version: [u8; 65] = [u8::default(); 65];
        value.version.iter().enumerate().for_each(|(i, val)| version[i] = *val as u8);

        let mut machine: [u8; 65] = [u8::default(); 65];
        value.machine.iter().enumerate().for_each(|(i, val)| machine[i] = *val as u8);

        let mut domainname: [u8; 65] = [u8::default(); 65];
        value.domainname.iter().enumerate().for_each(|(i, val)| domainname[i] = *val as u8);

        Self {
            sysname, 
            nodename, 
            release,
            version, 
            machine, 
            domainname
        }

    }
}

impl NameStructure {

    fn _sysname(&self) -> String {
        String::from(ex_str::from_utf8(&self.sysname).unwrap())
    }

    fn nodename(&self) -> String {
        String::from(ex_str::from_utf8(&self.nodename).unwrap())
    }

    fn _release(&self) -> String {
        String::from(ex_str::from_utf8(&self.release).unwrap())
    }

    fn _version(&self) -> String {
        String::from(ex_str::from_utf8(&self.version).unwrap())
    }

    fn _machine(&self) -> String {
        String::from(ex_str::from_utf8(&self.machine).unwrap())
    }

    fn _domainname(&self) -> String {
        String::from(ex_str::from_utf8(&self.domainname).unwrap())
    }
    
}

pub(crate) fn gethostname() -> String {
    let mut result: libc::utsname = unsafe { zeroed() };
    unsafe { libc::uname(&mut result) }; 
    NameStructure::from(result).nodename()
}

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
