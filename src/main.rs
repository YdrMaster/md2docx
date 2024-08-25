mod heading;
mod paragraph;
mod root;
mod strong;
mod style;
mod text;

use markdown::{mdast::Node, to_mdast, ParseOptions};
use std::fs::{read_to_string, File};

fn main() -> Result<(), BuildError> {
    let readme = read_to_string("README.md").unwrap();
    let Ok(Node::Root(root)) = to_mdast(&readme, &ParseOptions::gfm()) else {
        panic!("Failed to parse markdown");
    };
    println!("{root:#?}");
    let docx = root::from_root(root)?.build();
    let file = File::create("readme.docx").unwrap();
    docx.pack(file).unwrap();
    Ok(())
}

#[derive(Debug)]
struct BuildError;
