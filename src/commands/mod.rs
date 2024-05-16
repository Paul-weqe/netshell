pub(crate) mod operational_cmd;
pub(crate) mod configuration_cmd;


pub(crate) enum ClappedOutput {
    Completed,
    LevelDown, 
    LevelUp,
    ClearScreen,
    Logout
}

