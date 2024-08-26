mod list;
mod numbering;
mod paragraph;
mod root;
mod style;
mod text;

use docx_rs as docx;
use markdown::{
    mdast::{self as md, Node as Ast},
    to_mdast, ParseOptions,
};
use numbering::add_numbering;
use std::{
    borrow::Cow,
    fs::{read_to_string, File},
    path::Path,
};
use style::add_style;

fn main() {
    let readme = read_to_string("README.md").unwrap();
    let Ok(Ast::Root(root)) = to_mdast(&readme, &ParseOptions::gfm()) else {
        panic!("Failed to parse markdown");
    };
    println!("{root:#?}");
    let docx = root::from_root(root);
    let docx = add_style(docx);
    let docx = add_numbering(docx);
    let name = find_available_name(Path::new("readme.docx"));
    let file = File::create(name).unwrap();
    docx.build().pack(file).unwrap();
}

fn find_available_name(name: &Path) -> Cow<Path> {
    if !name.exists() {
        return name.into();
    }
    let dir = name.parent().unwrap();
    let name = name
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .strip_suffix(".docx")
        .unwrap();
    for i in 1.. {
        let name = dir.join(format!("{name} ({i}).docx"));
        if !name.exists() {
            return name.into();
        }
    }
    unreachable!()
}
