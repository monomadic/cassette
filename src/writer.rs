use std::io::{self, Write};
use crate::*;
use templar::*;

/// writes a project object to files on disk
pub fn write_project(project: Project, directory: String) -> CassetteResult<()> {

    Ok(())
}



impl XMLNode {
    pub fn write<W: Write>(&self, writer: &mut W) -> CassetteResult<()> {
        if self.ident == ("_TEXT") {
            writer.write(self.text.as_bytes())?;
            return Ok(());
        }
        writer.write(&format!("<{}", self.ident).as_bytes())?;
        writer.write(b">")?;

        for child in self.children.iter() {
            child.write(writer)?;
        }

        writer.write(&format!("</{}>", self.ident).as_bytes())?;

        Ok(())
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
