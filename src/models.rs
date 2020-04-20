use std::collections::HashMap;

#[derive(Debug)]
pub struct Project {
    styles: HashMap<String, Vec<Style>>,
    // scripts: Vec<Script>,
    // files: Vec<PathBuf>,
    pub documents: Vec<XMLNode>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            styles: HashMap::new(),
            documents: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Style {
    property: String,
    rule: String,
}

#[derive(Debug)]
pub struct XMLNode {
    pub ident: String,
    pub attributes: HashMap<String, Vec<String>>,
    pub terminated: bool,
    pub children: Vec<XMLNode>,
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
