use cassette::*;
use std::path::PathBuf;
use structopt::StructOpt;

mod files;

#[derive(StructOpt, Debug)]
#[structopt(name = "cassette")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,
}

fn main() {
    match run() {
        Ok(_) => println!("\n"),
        Err(e) => println!("\n\nERROR: {}", e),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let file_content = files::read_file(opt.source)?;

    let project = cassette::parse_str(&format!("{}\n", file_content))?;

    for document in project.documents {
        println!("{}", document.to_string()?);
    }

    Ok(())
}
