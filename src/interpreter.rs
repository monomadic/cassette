use templar::*;
use crate::*;
use std::collections::HashMap;
use std::io::{self, Write};

pub(crate) fn run(nodes: Vec<UnwoundNode>) -> CassetteResult<Project> {
    let mut project = Project::new();

    // extract html outputs

    // extract css outputs

    // extract js outputs

    // extract file outputs

    println!("DEBUG: {:?}", nodes);

    for node in nodes {
        //println!("{}({:?})", node.ident, node.properties);
        //call(&node.ident, node.locals)?;

        if node.ident == "html" {
            use std::io::{self, Write};
            let mut stdout = io::stdout();
            write_html(&mut stdout, node)?;
        }
    }

    Ok(project)
}

fn write_html<W: Write>(writer: &mut W, node: UnwoundNode) -> CassetteResult<()> {
    // note: account for inline styles + js
    writer.write(&format!("<{}{}>", node.ident, inline_styles(&node.children)?).as_bytes());
    for child in node.children {
        write_html(writer, child)?;
    }
    writer.write(&format!("</{}>", node.ident).as_bytes());

    Ok(())
}

fn inline_styles(children: &Vec<UnwoundNode>) -> CassetteResult<String>  {
    Ok("".into())
}
// /// recursively scan the tree and call a callback on any matches
// fn scan(ident: &str, cb: fn) -> Result<(), CassetteError> {

// }

fn call(ident: &str, args: HashMap<String, Property>) -> Result<(), CassetteError> {
    match &*ident {
        "print" => println!("{}", get_local("text", &args)?),
        _ => { return Err(CassetteError::FunctionNotFound(ident.into())); },
    };

    Ok(())
}

fn get_local(ident: &str, args: &HashMap<String, Property>) -> Result<Property, CassetteError> {
    args.get(ident).map(|p|p.clone()).ok_or(CassetteError::LocalNotFound(ident.into()))
}

fn get_string_property_at(properties: Vec<Property>, pos: usize) -> CassetteResult<String> {
    if let Some(Property::QuotedString(s)) = properties.get(0) {
        return Ok(s.into());
    } else {
        // fix this
        return Err(Box::new(CassetteError::UnknownBlock(format!("failed to get string at position {}", 0))));
    }
}

// note: upon extracting a series of html files, we need to post process to extract styles (inline and stdlib)
fn interpret_html_file(nodes: Vec<Node>) -> CassetteResult<HtmlDocument> {
    return Err(Box::new(CassetteError::ParameterMissing("ident".into(), "output_file".into())));
}

fn param_require(params: Vec<Property>, param: &str) -> CassetteResult<String> {
    Ok(" ".into())
}