use clap::Subcommand;

use crate::{commands::ClappedOutput, core, Context};


pub(super) fn execute(item: SetItem, context: &mut Context) -> std::io::Result<ClappedOutput>{
    match item {

        // config-mode# set system [param]
        SetItem::System { param } => {
            match param {

                // config-mode# set system host-name [hostname]
                System::HostName { hostname } => {
                    if core::sethostname(&hostname) >= 0 {
                        context.config.hostname = hostname.clone();
                        return Ok(ClappedOutput::Completed);
                    } 
                    else { 
                        return Ok(ClappedOutput::Completed);
                    }
                }
                System::Login { username, password }=> {
                    core::auth::create_user(username.as_str(), password.as_str());
                    Ok(ClappedOutput::Completed)
                }
            }
        }
    }
}



#[derive(Subcommand, Debug)]
pub(super) enum SetItem {
    // set system [param]
    System {
        #[command(subcommand)]
        param: System
    }
}

#[derive(Subcommand, Debug)]
enum System {

    // set system host-name [host-name]
    HostName {
        hostname: String
    },
    Login {
        username: String,
        password: String
    }
}