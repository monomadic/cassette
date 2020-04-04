use std::io::{self, Write};
use crate::*;
use templar::*;

/// writes a project object to files on disk
pub fn write_project(project: Project, directory: String) -> CassetteResult<()> {

    Ok(())
}

pub(crate) fn write_html_file(_file: &str, nodes: Vec<Node>) -> CassetteResult<()> {
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
                // check for inline styles

                writer.write(&format!("<{}{}>", ident, inline_styles(&children)?).as_bytes());
                write_html(writer, children)?;
                writer.write(&format!("</{}>", ident).as_bytes());
            },
            _ => (),
        }
    }

    Ok(())
}

fn inline_styles(children: &Vec<Node>) -> CassetteResult<String> {
    let mut styles: Vec<String> = Vec::new();

    // println!("--{:?}", children);

    for child in children {
        match child {
            Node::Assignment { ident, value } => {
                match &(**ident) {
                    "text-color" => styles.push(format!("color: {};", value)),
                    _ => (),
                }
            },
            _ => (),
        }
    };

    // println!("--{:?}", styles);

    if styles.is_empty() {
        Ok("".into())
    } else {
        Ok(format!(" style=\"{}\"", styles.join(", ")))
    }
}

fn get_symbol_property_at(properties: Vec<Property>, pos: usize) -> CassetteResult<String> {
    if let Some(Property::QuotedString(s)) = properties.get(0) {
        return Ok(s.into());
    } else {
        // fix this
        return Err(Box::new(CassetteError::UnknownBlock(format!("failed to get string at position {}", 0))));
    }
}
