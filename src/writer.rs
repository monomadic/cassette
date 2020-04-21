use std::io::Write;
use crate::*;

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

    pub fn to_string(&self) -> CassetteResult<String> {


        let mut buffer: Vec<u8> = Vec::new();
        self.write(&mut buffer)?;

        Ok(String::from_utf8(buffer)?)
    }
}

// fn get_symbol_property_at(properties: Vec<Property>, pos: usize) -> CassetteResult<String> {
//     if let Some(Property::QuotedString(s)) = properties.get(0) {
//         return Ok(s.into());
//     } else {
//         // fix this
//         return Err(Box::new(CassetteError::UnknownBlock(format!("failed to get string at position {}", 0))));
//     }
// }
