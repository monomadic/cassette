use std::error;
use std::io;

pub type CassetteResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum CassetteError {
    FunctionNotFound(String)
}

use std::fmt;
impl fmt::Display for CassetteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // TemplarError::IoError(ref e) => write!(f, "IO error: {}", e),
            CassetteError::FunctionNotFound(_) => write!(f, "Function Not Found"),
        }
    }
}

impl error::Error for CassetteError {
    fn description(&self) -> &str {
        match *self {
            // CassetteError::IoError(ref e) => e.description(),
            CassetteError::FunctionNotFound(ref f) => "Function Not Found",
        }
    }
}
