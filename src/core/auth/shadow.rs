
use std::{ffi::{CStr, CString}, time::{SystemTime, UNIX_EPOCH}};
use md5;

#[derive(Debug)]
/// Field descriptions from: https://linuxize.com/post/etc-shadow-file/
pub struct Shadow {
    // username 
    name: String,
    
    // password of the user
    // format: $[protocol]$[salt]$[hashed]
    // [protocol] -> {1 – MD5}
    //            -> {2a – Blowfish}
    //            -> {2y – Eksblowfish}
    //            -> {5 – SHA-256}
    //            -> {6 – SHA-512}
    password: String,

    // no of days since last password change
    // in epoch time
    last_change: i64,

    // minimum number of days that must pass for the user to be allowed to change 
    // their password.
    // Default 0
    min_pwd: i64,

    // maximum password age. Number of days after which the user password must be changed.
    // default is 99999 
    max_pwd: i64,

    // number of days before the password expires during which the user is warned 
    // that the password must be changed. Default 7
    warn: i64,
    
    // The number of days after the user password expires before the user account is disabled. 
    // Typically this field is empty.
    inactive: i64,

    // The date when the account was disabled. Represented in epoch date
    expire: i64,

    // Unused. This field is ignored. It is reserved for future use. 
    unused: u64
}

impl From<libc::spwd> for Shadow {
    fn from(value: libc::spwd) -> Self {
        unsafe {
            Self {
                name: CStr::from_ptr(value.sp_namp)
                    .to_str().expect("unable to convert libc::spwd::sp_namp to Shadow::name").to_string(),

                password:  CStr::from_ptr(value.sp_pwdp)
                    .to_str().expect("unable to convert libc::spwd::sp_pwdp to Shadow::password").to_string(),

                last_change: value.sp_lstchg,
                min_pwd: value.sp_min,
                max_pwd: value.sp_max,
                warn: value.sp_warn,

                // 
                inactive: value.sp_inact,
                expire: value.sp_expire,
                unused: value.sp_flag
            }
        }
    }
}

impl Into<libc::spwd> for Shadow {
    fn into(self) -> libc::spwd {
        libc::spwd {
            sp_namp: CString::new(self.name.as_str())
                .expect("unable to convert Shadow::name to libc::spwd::sp_namp").into_raw(),
            
            sp_pwdp: CString::new(self.password.as_str())
                .expect("unabel to convert Shadow::password to libc::spwd::sp_pwdp").into_raw(),
            
            sp_lstchg: self.last_change,
            sp_min: self.min_pwd,
            sp_max: self.max_pwd,
            sp_warn: self.warn,
            sp_inact: self.inactive,
            sp_expire: self.expire,
            sp_flag: self.unused
        }
    }
}
