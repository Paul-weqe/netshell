use crate::base;


#[derive(Clone)]
pub(crate) struct OperationMode { 
    pub(crate) prompt: String 
}

impl Default for OperationMode {
    fn default() -> Self {
        Self { prompt: format!("{}>", base::gethostname()) }
    }
}

#[derive(Clone)]
pub(crate) struct ConfigMode { 
    pub(crate) prompt: String 
}

impl Default for ConfigMode {
    fn default() -> Self {
        Self { prompt: format!("{}#", base::gethostname()) }
    }
}