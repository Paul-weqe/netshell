use clap::Subcommand;

use crate::{commands::ClappedOutput, ifaces, Context};


pub(super) fn execute(item: ShowItem, _context: &Context) -> std::io::Result<ClappedOutput> {
    match item {
        ShowItem::Interfaces => {
            ifaces::draw_interface(pnet::datalink::interfaces(), true);
            Ok(ClappedOutput::Completed)
        }
    }
}

#[derive(Subcommand, Debug)]
pub(super) enum ShowItem {
    Interfaces
}