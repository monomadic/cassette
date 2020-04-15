use std::error;
use std::io;

pub type CassetteResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum CassetteError {
    FunctionNotFound(String),
    LocalNotFound(String),
    UnknownBlock(String),
    UnknownStyle(String, String),
    ParameterMissing(String, String),
}

use std::fmt;
impl fmt::Display for CassetteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // TemplarError::IoError(ref e) => write!(f, "IO error: {}", e),
            CassetteError::FunctionNotFound(ref t) => write!(f, "unknown function: {}", t),
            CassetteError::LocalNotFound(ref t) => write!(f, "unknown local: {}", t),
            CassetteError::UnknownBlock(ref t) => write!(f, "unknown block type: {}", t),
            CassetteError::UnknownStyle(ref n, ref p) => write!(f, "unknown style type: {} {}", n, p),
            CassetteError::ParameterMissing(ref n, ref p) => write!(f, "Node '{}' is missing a required parameter: {}", n, p),
        }
    }
}

impl error::Error for CassetteError {
    // fn description(&self) -> &str {
    //     &format!("{}", self)
    // }
}
