use std::error;
use std::io;

pub type CassetteResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum CassetteError {
    // FunctionNotFound(String),
    UnknownBlock(String),
    ParameterMissing(String, String),
}

use std::fmt;
impl fmt::Display for CassetteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // TemplarError::IoError(ref e) => write!(f, "IO error: {}", e),
            CassetteError::UnknownBlock(ref t) => write!(f, "Unknown Block Type: {}", t),
            CassetteError::ParameterMissing(ref n, ref p) => write!(f, "Node '{}' is missing a required parameter: {}", n, p),
        }
    }
}

impl error::Error for CassetteError {
    // fn description(&self) -> &str {
    //     &format!("{}", self)
    // }
}
