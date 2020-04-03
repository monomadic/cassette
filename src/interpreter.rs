use templar::*;
use crate::*;

pub(crate) fn interpret_node_tree(nodes: Vec<Node>) -> CassetteResult<Project> {
    let mut project = Project::new();

    // FIRST PASS: remove resource (global style, script, file) nodes from the tree

    // SECOND PASS: render html documents remaining

    // these are all top level nodes, which can be a different instruction set to lower level nodes.
    for node in nodes {
        match node {
            Node::Block { ident, properties, children } => {
                // cascade variables down the chain
                // execute children first (leaf nodes up)
                // interpret_node_tree(children)?;
                // finally, execute the block
                // execute_block(ident, properties)?;

                match &*ident {
                    "page" => {
                        let file = get_string_property_at(properties, 0)?; // fix this soon to use unwound nodes
                        // let file = properties.get(0).expect(&format!("file missing: {:?}", properties));
                        writer::write_html_file(&file, children)?;
                        
                        // project.documents.push(interpret_html_file(children)?);
                    },
                    _ => return Err(Box::new(CassetteError::UnknownBlock(ident))),
                }
                
            },
            _ => (),
        }
    }

    Ok(project)
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

// fn execute_block(ident: &str, properties: Vec<Property>) -> CassetteResult<()> {
// }


// pub(crate) struct CassetteInterpreter;

// impl CassetteInterpreter {

//     pub fn new() -> Self {
//         Self {}
//     }

//     pub fn walk(&self, nodes: Vec<Node>) -> CassetteResult<()> {
//         for node in nodes {
//             match node {
//                 Node::Block { ident, properties, children } => {
//                     // execute children first (leaf nodes up)
//                     self.walk(children)?;
//                     // pass any variables up the chain (or do they go down?)

//                     // finally, execute the block
//                     // println!("-- {}", ident);
//                     self.execute(&ident, properties)?;
//                 },
//                 _ => (),
//             }
//         }

//         Ok(())
//     }

//     // fn execute(&self, function_name: &str, properties: Vec<Property>) -> CassetteResult<()> {
//     //     match(function_name) {
//     //         "h1" => println!("h1 printed"),
//     //         _ => return Err(Box::new(CassetteError::UnknownBlock(function_name))),
//     //     };
//     //     return Ok(())
//     // }
// }
