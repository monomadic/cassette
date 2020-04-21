mod error;
mod interpreter;
mod models;
mod writer;
pub use error::*;
pub(crate) use models::*;

pub use error::CassetteError;
pub type CassetteResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn parse_str(i: &str) -> CassetteResult<Project> {
    let nodes = templar::parse_str(i)?;
    interpreter::run(nodes)
}
