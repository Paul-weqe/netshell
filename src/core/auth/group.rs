use std::{ffi::{CStr, CString}, io, slice};

use libc;

#[derive(Debug)]
pub struct Group {
    // The group ID of the group.
    id: u32,

    // The name of the group.
    name: String,

    // A vector of pointers to the names of users in the group. 
    // Each user name is a String  
    members: Vec<String>
}

impl From<libc::group> for Group {
    fn from(value: libc::group) -> Self {
        unsafe {

            let mut members: Vec<String> = Vec::new();
            let c_mem = value.gr_mem;

            // using abitrary numbr (512) since it is proving difficult to get 
            // the actual length of a *mut *mut i8 (c_mem)
            let sl = slice::from_raw_parts_mut(
                c_mem, 1024
            ).to_owned().to_vec();

            for x in sl.clone() {
                if x == std::ptr::null_mut() {
                    break
                }
                members.push(CStr::from_ptr(x).to_str().unwrap().to_string());
            }
            Self {
                id: value.gr_gid,
                name: CStr::from_ptr(value.gr_name)
                    .to_str().expect("unable to convert libc::group::gr_name to Group::name").to_string(),
                members
            }
        }
    }
}

pub fn get_group(uname: &str) -> io::Result<Group> {
    let u = CString::new(uname)
        .expect("unable to convert &str to *mut i8").into_raw();
    let gr_ptr = unsafe {
        libc::getgrnam(u)
    };

    if gr_ptr == std::ptr::null_mut() {
        println!("-------------");
        panic!("Unable to convert!")
    }
    Ok(Group::from(unsafe{ *gr_ptr }))

}