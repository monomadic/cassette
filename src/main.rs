mod files;
mod error;
pub use error::*;

fn main() {
   let file = files::read_file(std::path::PathBuf::from("examples/projects/basic.cassette")).unwrap();
   println!("{:?}", templar::parse(&file));
}
