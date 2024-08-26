mod heading;
mod numbering;
mod paragraph;
mod root;
mod style;
mod text;

use markdown::{mdast::Node, to_mdast, ParseOptions};
use numbering::add_numbering;
use std::{
    fs::{read_to_string, File},
    path::{Path, PathBuf},
};
use style::add_style;

fn main() {
    let readme = read_to_string("README.md").unwrap();
    let Ok(Node::Root(root)) = to_mdast(&readme, &ParseOptions::gfm()) else {
        panic!("Failed to parse markdown");
    };
    // println!("{root:#?}");
    let docx = root::from_root(root);
    let docx = add_style(docx);
    let docx = add_numbering(docx);
    let name = Path::new("readme.docx");
    let file = if !name.exists() {
        File::create(name).unwrap()
    } else {
        let mut i = 1;
        loop {
            let name = PathBuf::from(format!("readme ({i}).docx"));
            if !name.exists() {
                break File::create(name).unwrap();
            }
            i += 1;
        }
    };
    docx.build().pack(file).unwrap();
}
