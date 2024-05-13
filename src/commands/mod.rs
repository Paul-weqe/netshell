pub(crate) mod opr_mode;
pub(crate) mod conf_mode;


pub(crate) enum ParsedOutput {
    Completed,
    LevelDown, 
    LevelUp
}