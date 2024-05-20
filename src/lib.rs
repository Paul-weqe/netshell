pub mod routing;
pub mod cli;
mod core;

mod commands;
mod ifaces;
mod config;
mod modes;

pub use core::auth;
use crate::core::history::History;

pub const REGULAR_USER_SHELL: &str = "/bin/netshell";
pub const NETSHELL_HOME_DIR: &str = "/home/netshell";
pub const HISTORY_FILE: &str = "/root/.netsh_history";

// auth files
pub const PASSWD_FILE: &str = "/etc/passwd";
pub const GROUP_FILE: &str = "/etc/group";
pub const NETSHELL_GROUP_ID: u32 = 1738;


#[derive(Default, Clone)]
pub struct Context {
    pub mode: cli::Mode, 
    config: Configuration,
    history: History
}

#[derive(Clone)]
pub(crate) struct Configuration {
    hostname: String
}

impl Default for Configuration {
    fn default() -> Self {
        Self { hostname: core::gethostname() }
    }
}