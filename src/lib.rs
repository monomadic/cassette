mod files;
mod error;
mod interpreter;
mod writer;
mod models;
pub use error::*;
pub(crate) use models::*;

pub fn parse_str(i: &str) -> CassetteResult<Project> {
    let nodes = templar::parse_str(i)?;
    interpreter::run(nodes)
}
