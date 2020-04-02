use templar::*;
use crate::*;

pub(crate) struct CassetteInterpreter;

impl CassetteInterpreter {

    pub fn new() -> Self {
        Self {}
    }

    pub fn walk(&self, nodes: Vec<Node>) -> CassetteResult<()> {
        for node in nodes {
            match node {
                Node::Block { ident, properties, children } => {
                    // execute children first (leaf nodes up)
                    self.walk(children)?;
                    // pass any variables up the chain (or do they go down?)

                    // finally, execute the block
                    // println!("-- {}", ident);
                    self.execute(&ident)?;
                },
                _ => (),
            }
        }

        Ok(())
    }

    // fn variable_assignment(assignment: ) {

    // }

    fn execute(&self, function_name: &str) -> CassetteResult<()> {
        match(function_name) {
            "h1" => println!("h1 printed"),
            _ => return Err(Box::new(CassetteError::FunctionNotFound(function_name.to_string()))),
        };
        return Ok(())
    }
}
