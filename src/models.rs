use std::collections::HashMap;

pub struct Project {
    styles: HashMap<String, Vec<Style>>,
    // scripts: Vec<Script>,
    // files: Vec<PathBuf>,
    documents: Vec<HtmlDocument>,
}

struct Style {
    property: String,
    rule: String,
}

struct HtmlDocument {
    includes: Vec<FileInclude>,
    path: String,
    title: String,
    tags: Vec<HtmlTag>,
}

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
