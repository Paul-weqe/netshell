pub mod routing;
pub mod cli;

mod commands;
mod core;
mod ifaces;
mod config;
mod modes;

use crate::core::history::History;

pub const DEFAULT_HISTORY_LOCATION: &str = "/root/.netsh_history";

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