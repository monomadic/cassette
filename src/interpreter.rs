use templar::*;
use crate::*;

pub(crate) fn run(nodes: Vec<UnwoundNode>) -> CassetteResult<Project> {
    let mut project = Project::new();

    // FIRST PASS: remove resource nodes and function declarations from the tree
    println!("nodes: {:#?}", nodes);

    // SECOND PASS: render html documents remaining
    // println!("{:#?}", &nodes);
    // these are all top level nodes, which can be a different instruction set to lower level nodes.
    for node in nodes {
        println!("{}({:?})", node.ident, node.properties);
        call(&node.ident, node.properties)?;
    }

    Ok(project)
}

fn call(ident: &str, args: Vec<Property>) -> CassetteResult<()> {
    match &*ident {
        "print" => println!("{}", get_property_at(args.clone(), 0)?),
        _ => (),
    };
    println!("calling function: {} {:?}", ident, args);
    Ok(())
}

fn get_property_at(properties: Vec<Property>, pos: usize) -> CassetteResult<Property> {
    if let Some(s) = properties.get(0) {
        return Ok(s.clone());
    } else {
        return Err(Box::new(CassetteError::UnknownBlock(format!("failed to get param at position {}", 0))));
    }
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
