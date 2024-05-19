use std::ffi::CString;


pub mod shadow;
mod passwd;
pub mod group;

pub fn create_user(username: &str, password: &str) {
    passwd::Passwd::create_passwd(username, format!("/home/{username}").as_str(), 1738);
}

pub fn get_user_shadow(uname: &str) -> shadow::Shadow {
    let c_uname = CString::new(uname).unwrap();
    
    let sp = unsafe { 
       *libc::getspnam(c_uname.into_raw())
    };
    shadow::Shadow::from(sp)
}
