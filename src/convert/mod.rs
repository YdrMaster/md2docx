mod list;
mod numbering;
mod paragraph;
mod root;
mod style;
mod table;
mod text;

use docx_rs as docx;
use markdown::{
    mdast::{self as md, Node as Ast},
    to_mdast, ParseOptions,
};
use numbering::add_numbering;
use std::{
    fs::{read_to_string, File},
    path::PathBuf,
};
use style::add_style;

#[derive(Args, Default)]
pub struct Args {
    /// The file to convert
    file: PathBuf,
    /// A Toml style file
    #[clap(short, long)]
    style: Option<PathBuf>,
}

impl Args {
    pub fn convert(self) {
        let Self { file, style } = self;
        let dir = file.parent().unwrap();
        let name = file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".md")
            .unwrap();
        let style = style
            .map(|p| {
                read_to_string(&p)
                    .unwrap_or_else(|e| panic!("Failed to read style file: {e}"))
                    .parse::<toml::Table>()
                    .unwrap_or_else(|e| {
                        panic!("Failed to parse style file, must be in TOML format: {e}")
                    })
            })
            .unwrap_or_default();

        let md = read_to_string(&file).unwrap();
        let Ok(Ast::Root(root)) = to_mdast(&md, &ParseOptions::gfm()) else {
            panic!("Failed to parse markdown");
        };
        let docx = root::from_root(root, dir);
        let docx = add_style(docx, style);
        let docx = add_numbering(docx);

        let name = find_available_name(name);
        let file = File::create(name).unwrap();
        docx.build().pack(file).unwrap();
    }
}

fn find_available_name(name: &str) -> PathBuf {
    let path = PathBuf::from(format!("{name}.docx"));
    if !path.exists() {
        return path;
    }
    for i in 1.. {
        let path = PathBuf::from(format!("{name} ({i}).docx"));
        if !path.exists() {
            return path;
        }
    }
    unreachable!()
}
