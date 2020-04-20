use templar::*;
use crate::*;
use std::collections::HashMap;
use std::io::{self, Write};

pub(crate) fn run(nodes: Vec<UnwoundNode>) -> CassetteResult<Project> {
    use std::io::{self, Write};
    let mut stdout = io::stdout();
    let mut project = Project::new();

    // extract html outputs

    // extract css outputs

    // extract js outputs

    // extract file outputs

    println!("NODES: {:#?}\n", nodes);

    for node in nodes.clone() {
        if node.ident == "page" {
            // set a new writer
            for child in node.children.clone() {
                if let Some(child) = extract_html(child)? {
                    project.documents.push(child);
                }
            }
        }
    }

    Ok(project)
}

fn write_html<W: Write>(writer: &mut W, node: UnwoundNode) -> CassetteResult<()> {
    // note: account for inline styles + js
    if node.ident == "tag" {
        let tag_type = node.get_local("type").ok_or(CassetteError::LocalNotFound(String::from("type")))?;
        writer.write(&format!("<{}{}>", tag_type,
            inline_styles(&node.children)?
        ).as_bytes())?;
    }
    for child in node.children.clone() {
        write_html(writer, child)?;
    }
    if node.ident == "tag" {
        let tag_type = node.get_local("type").ok_or(CassetteError::LocalNotFound(String::from("type")))?;
        writer.write(&format!("</{}>", tag_type).as_bytes())?;
    }

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
fn extract_html(node: UnwoundNode) -> CassetteResult<Option<XMLNode>> { // todo: should be result-option
    // note: account for inline styles + js
    println!("TAG: {:?}", node);

    match &(*node.ident) {
        "tag" => {
            let ident = node.get_local("type").ok_or(CassetteError::LocalNotFound(String::from("type")))?;
            let mut children = Vec::new();

            for child in node.children {
                if let Some(child) = extract_html(child)? {
                    children.push(child);
                }
            }

            return Ok(Some(XMLNode {
                ident: format!("{}", ident),
                attributes: HashMap::new(),
                terminated: false,
                children
            }));
        },
        "_TEXT" => {
            return Ok(Some(XMLNode {
                ident: "_TEXT".into(),
                attributes: HashMap::new(),
                terminated: false,
                children: Vec::new(),
            }))
        },
        _ => (),
    }


    Ok(None) // intentional. no errors found, but also no nodes.
}

fn param_require(params: Vec<Property>, param: &str) -> CassetteResult<String> {
    Ok(" ".into())
}