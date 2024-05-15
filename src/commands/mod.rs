pub(crate) mod opr_commands;
pub(crate) mod conf_commands;
pub(crate) mod edit_conf_commands;


pub(crate) enum ClappedOutput {
    Completed,
    LevelDown, 
    LevelUp,
    Logout
}