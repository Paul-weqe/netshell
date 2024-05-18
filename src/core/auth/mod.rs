use std::ffi::{CStr, CString};

mod passwd;


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
    inactive: Option<i64>,

    // The date when the account was disabled. Represented in epoch date
    expire: Option<i64>,

    // Unused. This field is ignored. It is reserved for future use. 
    unused: Option<i64>
}

impl From<libc::spwd> for Shadow {
    fn from(value: libc::spwd) -> Self {
        unsafe {
            Self {
                name: CStr::from_ptr(value.sp_namp)
                    .to_str().expect("unable to convert libc::spwd::sp_namp to Shadow::name").to_string(),

                password:  CStr::from_ptr(value.sp_pwdp)
                    .to_str().expect("unable to convert libc::spwd::sp_pwdp to Shadow::password").to_string(),

                last_change: (value.sp_lstchg) as i64,
                min_pwd: (value.sp_min) as i64,
                max_pwd: (value.sp_max) as i64,
                warn: (value.sp_warn) as i64,

                // 
                inactive: if (value.sp_inact) as i64 == -1 { None } else { Some(value.sp_inact as i64) },
                expire: if (value.sp_expire) as i64 == -1 { None } else { Some(value.sp_expire as i64) },
                unused: if (value.sp_flag) as i64 == -1 { None } else { Some(value.sp_flag as i64) }
            }
        }
    }
}




pub fn create_user(username: &str, password: &str) {
    passwd::Passwd::create_passwd(username, format!("/home/{username}").as_str(), 1738);
}

pub fn get_user_shadow(uname: &str) -> Shadow {
    let c_uname = CString::new(uname).unwrap();
    
    let sp = unsafe { 
       *libc::getspnam(c_uname.into_raw())
    };
    Shadow::from(sp)
}
