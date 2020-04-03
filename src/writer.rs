use std::io::{self, Write};
use crate::*;
use templar::*;

/// writes a project object to files on disk
pub fn write_project(project: Project, directory: String) -> CassetteResult<()> {

    Ok(())
}

pub(crate) fn write_html_file(_file: &str, nodes: Vec<Node>) -> CassetteResult<()> {
    // println!("--{:#?}", nodes);
    use std::io::{self, Write};
    let mut stdout = io::stdout();
    writer::write_html(&mut stdout, nodes)?;
    Ok(())
}

// /// this will be deleted.
pub fn write_html<W>(writer: &mut W, nodes: Vec<Node>) -> CassetteResult<()>
    where W : Write {
    for node in nodes {
        match node {
            Node::Assignment { ident, value } => {
                if ident == "text" {
                    writer.write(&format!("{}", value).as_bytes()); // note: escape text here
                }
            }
            Node::Block { ident, properties, children } => {
                writer.write(&format!("<{}>", ident).as_bytes());
                write_html(writer, children)?;
                writer.write(&format!("</{}>", ident).as_bytes());
            },
            _ => (),
        }
    }

    Ok(())
}
