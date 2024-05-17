pub mod routing;
pub mod cli;
pub mod core;

mod commands;
mod ifaces;
mod config;
mod modes;

use crate::core::history::History;

pub const REGULAR_USER_SHELL: &str = "/bin/bash";
pub const NETSHELL_HOME_DIR: &str = "/home/netshell";
pub const NETSHELL_GROUP_NUMBER: u32 = 1500;
pub const HISTORY_FILE: &str = "/root/.netsh_history";


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