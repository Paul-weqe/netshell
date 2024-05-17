use crate::core;


#[derive(Clone)]
pub struct OperationMode { 
    pub(crate) prompt: String 
}

impl Default for OperationMode {
    fn default() -> Self {
        Self { prompt: format!("{}>", core::gethostname()) }
    }
}

#[derive(Clone)]
pub struct ConfigMode { 
    pub(crate) prompt: String 
}

impl Default for ConfigMode {
    fn default() -> Self {
        Self { prompt: format!("{}#", core::gethostname()) }
    }
}