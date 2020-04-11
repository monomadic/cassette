mod files;
mod error;
mod interpreter;
mod writer;
mod models;
pub use error::*;
pub(crate) use models::*;

fn main() {
    match run() {
        Ok(_) => println!("\n"),
        Err(e) => println!("\n\nERROR: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = files::read_file(std::path::PathBuf::from("examples/projects/functions.cassette"))?;
    let (_, nodes) = templar::parser::run(&format!("{}\n", file_content)).unwrap();
    let nodes = templar::postprocessor::run(nodes);

    // println!("result: {:?}", e);
    // let interpreter     = interpreter::CassetteInterpreter::new();

    // interpreter.walk(result)?;
    // let graph           = templar::interpreter::run(result)?;
    // let output          = generator::run(graph)?;
    // let _               = output.write_all()?;

    // println!("{:#?}", templar::interpreter::run(result));

    use std::io::{self, Write};
    let mut stdout = io::stdout();
    // writer::write_html(&mut stdout, nodes)?;

    // interpreter::interpret_node_tree(nodes)?;

    Ok(())
}
