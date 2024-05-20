use std::{ffi::CString, fs};

mod shadow;
mod passwd;
mod group;

pub use group::Group;
pub use shadow::Shadow;
pub use passwd::Passwd;

use crate::NETSHELL_GROUP_ID;

pub fn create_user(username: &str, password: &str) {
    let uid = 1739;
    passwd::Passwd::create_passwd(
        username, format!("/home/{username}").as_str(), uid, NETSHELL_GROUP_ID
    );
    Shadow::create(username, password);
    let home_dir = format!("/home/{username}");

    // CREATE USERS HOME DIRECTORY
    let _ = fs::create_dir(home_dir.clone());
    chown(&home_dir, uid, NETSHELL_GROUP_ID);
}

pub fn get_user_shadow(uname: &str) -> shadow::Shadow {
    let c_uname = CString::new(uname).unwrap();
    
    let sp = unsafe { 
       *libc::getspnam(c_uname.into_raw())
    };
    shadow::Shadow::from(sp)
}


pub fn chown(pathname: &str, owner: u32, group: u32){
    let c_pname = CString::new(pathname)
        .expect("unabel to change 'pathname' to CString").into_raw();
    let output = unsafe {
        libc::chown(c_pname, owner, group)
    };
}