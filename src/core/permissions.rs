use std::ffi::CString;
use libc;

#[derive(Default)]
pub struct Passwd {
    pw_name: String,
    pw_passwd: String,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: String,
    pw_dir: String,
    pw_shell: String
}

impl Passwd {

    pub fn create_user(username: &str, home_dir: &str, uid: u32) {
        let mut n_passwd: Passwd = Passwd::default();
        n_passwd.pw_dir = home_dir.to_string();
        n_passwd.pw_gecos = username.to_string();
        n_passwd.pw_name = username.to_string();
        n_passwd.pw_gid = uid;
        n_passwd.pw_uid = uid;
        n_passwd.pw_shell = "/bin/bash".to_string();
        
        unsafe {
            let fname = CString::new("/etc/passwd")
                .expect("unabel to convert '/etc/passwd' to cstring").into_raw();
            let mode = CString::new("a")
                .expect("unable to convert 'a' to cstring").into_raw();

            let f = libc::fopen(fname, mode);
            libc::putpwent(&n_passwd.into(), f);
            libc::fclose(f);
        };
        
    }
}

impl From<libc::passwd> for Passwd {
    fn from(value: libc::passwd) -> Self {
        unsafe {
            Self {
                pw_name: CString::from_raw(value.pw_name).into_string().expect(""),
                pw_passwd: CString::from_raw(value.pw_passwd).into_string().expect(""),
                pw_uid: value.pw_uid,
                pw_gid: value.pw_gid,
                pw_gecos: CString::from_raw(value.pw_gecos).into_string().expect(""),
                pw_dir: CString::from_raw(value.pw_dir).into_string().expect(""),
                pw_shell: CString::from_raw(value.pw_shell).into_string().expect("")
            }
        }
    }
}

impl Into<libc::passwd> for Passwd {
    fn into(self) -> libc::passwd {

        libc::passwd {
            pw_name: CString::new(self.pw_name.as_str())
                .expect("unable to convert Passwd.pw_name to libc::passwd.pw_name").into_raw(),
            
            pw_passwd: CString::new(self.pw_passwd.as_str())
                .expect("unable to convert Passwd.pw_passwd into libc::passwd.pw_passwd").into_raw(),
            
            pw_uid: self.pw_uid,
            pw_gid: self.pw_gid,

            pw_gecos: CString::new(self.pw_gecos.as_str())
                .expect("unable to convert Passwd.pw_gecos into libc::passwd.pw_gecos").into_raw(),
            
            pw_dir: CString::new(self.pw_dir.as_str())
                .expect("unable to convert Passwd.pw_dir into libc::passwd.pw_dir").into_raw(),
            pw_shell: CString::new(self.pw_shell.as_str())
                .expect("unable to convert Passwd.pw_dir into libc::passwd.pw_dir").into_raw()
        }
        
    }
}