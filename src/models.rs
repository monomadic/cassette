use std::collections::HashMap;

pub struct Project {
    styles: HashMap<String, Vec<Style>>,
    // scripts: Vec<Script>,
    // files: Vec<PathBuf>,
    documents: Vec<XMLFile>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            styles: HashMap::new(),
            documents: Vec::new(),
        }
    }
}

struct Style {
    property: String,
    rule: String,
}

pub struct XMLFile {
    path: std::path::PathBuf,
    nodes: Vec<XMLNode>,
}

pub struct XMLNode {
    ident: String,
    attributes: HashMap<String, Vec<String>>,
    terminated: bool,
    children: Vec<XMLNode>,
}

// impl XMLFile {
//     pub fn write
// }

enum FileInclude {
    Style(String),
    Script(String),
}

struct HtmlTag {
    ident: String,
    classes: Vec<String>,
    terminated: bool,
    content: Vec<HtmlTag>,
}

// struct Command {
//     description: String,
//     arguments: Vec<templar::Parameter>,
//     execution: Box<Fn()>,
// }
