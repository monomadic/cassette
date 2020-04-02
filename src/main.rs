mod files;
mod error;
mod interpreter;
pub use error::*;

fn main() {
    match run() {
        Ok(_) => println!("done."),
        Err(e) => println!("error: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file_content    = files::read_file(std::path::PathBuf::from("examples/projects/basic.cassette"))?;
    let (e, result)     = templar::parser::run(&file_content).unwrap();
    println!("result: {:?}", e);
    let interpreter     = interpreter::CassetteInterpreter::new();

    interpreter.walk(result)?;
    // let graph           = templar::interpreter::run(result)?;
    // let output          = generator::run(graph)?;
    // let _               = output.write_all()?;

    // println!("{:#?}", templar::interpreter::run(result));

    Ok(())
}
