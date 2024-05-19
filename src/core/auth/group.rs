use std::{ffi::{CStr, CString}, slice, mem};

use libc;

use crate::GROUP_FILE;

#[derive(Debug, Clone)]
pub struct Group {
    // The group ID of the group.
    id: u32,

    // password for the group. Normally blank.
    // Used mostly for privileged groups
    password: String,
    
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
                password: CStr::from_ptr(value.gr_passwd)
                    .to_str().expect("unable to convert libc::group::gr_passwd to Group::password").to_string(),
                members
            }
        }
    }
}

impl Into<libc::group> for Group {
    fn into(self) -> libc::group {
        // using 512 as the abitrary number, number of users 
        let mut m: [*mut i8; 512] = [unsafe{mem::zeroed()}; 512];
        
        self.members.iter().enumerate().for_each(|(index, a)| {
            m[index] = CString::new(a.as_str()).unwrap().into_raw();
        });

        libc::group {
            gr_gid: self.id,
            gr_name: CString::new(self.name.as_str()).unwrap().into_raw(),
            gr_passwd: CString::new("").unwrap().into_raw(),
            gr_mem: m.as_mut_ptr()
        }
    }
}

impl Group {

    /// creates a group if it does not exist. 
    /// the id will be generated
    pub fn create_group(name: &str, id: u32) -> Option<Group> {

        let group = Group {
            id,
            name: String::from(name),
            password: String::from(""),
            members: Vec::new()
        };

        // make sure the group does not exist
        if let None = Self::get_by_name(name) {
            unsafe {
                let fname = CString::new(GROUP_FILE)
                    .expect("unable to convert '/etc/group' to CString").into_raw();
                let mode = CString::new("a")
                    .expect("Unable to convert 'a' to CString").into_raw();
                let f = libc::fopen(fname, mode);
        
                let g: libc::group = group.clone().into();
        
                libc::putgrent(&g, f);
                libc::fclose(f);
            }
            return Some(group)
        }
        None
    }


    pub fn get_by_name(name: &str) -> Option<Group> {
        let group_name = CString::new(name)
            .expect("unable to convert &str to *mut i8").into_raw();
        let gr_ptr = unsafe {
            libc::getgrnam(group_name)
        };

        if gr_ptr == std::ptr::null_mut() {
            return None
        }
        Some(Group::from(unsafe{ *gr_ptr }))
    }

}

