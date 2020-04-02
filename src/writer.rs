use std::io::{self, Write};
use crate::*;
use templar::*;

/// writes a project object to files on disk
pub fn export_project(project: Project, directory: String) -> CassetteResult<()> {

    Ok(())
}

pub fn write_html<W>(writer: &mut W, nodes: Vec<Node>) -> CassetteResult<()>
    where W : Write {
    for node in nodes {
        match node {
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
