use templar::*;
use crate::*;
use std::collections::HashMap;

pub(crate) fn run(nodes: Vec<UnwoundNode>) -> CassetteResult<Project> {
    let mut project = Project::new();

    for node in nodes.clone() {
        if node.ident == "page" {
            // set a new writer
            for child in node.children.clone() {
                if let Some(child) = extract_html(child)? {
                    project.documents.push(child.clone());
                    // child.write(&mut stdout)?;
                }
            }

        }
    }

    Ok(project)
}

// note: upon extracting a series of html files, we need to post process to extract styles (inline and stdlib)
fn extract_html(node: UnwoundNode) -> CassetteResult<Option<XMLNode>> {
    // note: account for inline styles + js

    match &(*node.ident) {
        "tag" => {
            let ident = node.get_local("type").ok_or(CassetteError::LocalNotFound(String::from("type")))?;
            let mut attributes: HashMap<String, String> = HashMap::new();
            if let Some(style) = extract_inline_styles(&node)? {
                attributes.insert("style".to_string(), style);
            }

            let mut children = Vec::new();

            for child in node.children {
                if let Some(child) = extract_html(child)? {
                    children.push(child);
                }
            }

            return Ok(Some(XMLNode {
                ident: ident.to_string(),
                attributes: attributes,
                terminated: false,
                text: String::new(),
                children
            }));
        },
        "_TEXT" => {
            let text = node.get_local("text").ok_or(CassetteError::LocalNotFound(String::from("text")))?;

            return Ok(Some(XMLNode {
                ident: "_TEXT".into(),
                attributes: HashMap::new(),
                terminated: false,
                text: text.to_string(),
                children: Vec::new(),
            }))
        },
        // _ => { return Err(Box::new(CassetteError::LocalNotFound(String::from(node.ident)))); },
        _ => ()
    };


    Ok(None) // intentional. no errors found, but also no nodes.
}

fn extract_inline_styles(node: &UnwoundNode) -> CassetteResult<Option<String>> {
    let mut styles = Vec::new();

    if let Some(background_color) = node.get_local("background-color") {
        styles.push(format!("background-color: {}", background_color));
    }

    if styles.len() == 0 {
        return Ok(None);
    }

    Ok(Some(styles.join("; ")))
}