use image::ImageReader;
use markdown::{
    mdast::{
        BlockQuote, Code, Delete, Emphasis, Heading, Image, InlineCode, Link, List, ListItem,
        Node as Ast, Paragraph, Root, Strong, Table, TableCell, TableRow, Text, ThematicBreak,
    },
    to_mdast, ParseOptions,
};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

#[derive(Args, Default)]
pub struct Args {
    /// The file to show
    file: PathBuf,
}

impl Args {
    pub fn show(self) {
        let path = self.file;
        let md = read_to_string(&path).unwrap();
        let Ok(Ast::Root(root)) = to_mdast(&md, &ParseOptions::gfm()) else {
            panic!("Failed to parse markdown");
        };
        let mut lines = vec![false];
        print_ast(Ast::Root(root), path.parent().unwrap(), &mut lines);
        println!();
        println!();
    }
}

fn print_ast(ast: Ast, dir: &Path, lines: &mut Vec<bool>) {
    match ast {
        Ast::Root(Root { children, .. }) => {
            print!("{}Root", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Heading(Heading {
            children, depth, ..
        }) => {
            print!("{}Heading {depth}", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Paragraph(Paragraph { children, .. }) => {
            print!("{}Paragraph", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Link(Link { children, url, .. }) => {
            print!("{}Link: {url}", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::List(List {
            children,
            ordered,
            start,
            ..
        }) => {
            print!("{}List ", indent(lines));
            match (ordered, start) {
                (true, Some(start)) => print!("{start}.."),
                (false, None) => {}
                (_, _) => panic!(),
            }
            assert!(children.iter().all(|ast| matches!(ast, Ast::ListItem(_))));
            print_children(children, dir, lines);
        }
        Ast::ListItem(ListItem { children, .. }) => {
            print!("{}Item", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Text(Text { value, .. }) => {
            print!("{}Text: {value:?}", indent(lines));
        }
        Ast::InlineCode(InlineCode { value, .. }) => {
            print!("{}InlineCode: {value:?}", indent(lines));
        }
        Ast::Delete(Delete { children, .. }) => {
            print!("{}Delete", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Strong(Strong { children, .. }) => {
            print!("{}Strong", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Emphasis(Emphasis { children, .. }) => {
            print!("{}Emphasis", indent(lines));
            print_children(children, dir, lines);
        }

        Ast::BlockQuote(BlockQuote { children, .. }) => {
            print!("{}BlockQuote", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Code(Code {
            value, lang, meta, ..
        }) => print!(
            "{}Code(lang={lang:?} meta={meta:?}): {value:?}",
            indent(lines)
        ),
        Ast::ThematicBreak(ThematicBreak { .. }) => {
            print!("{}ThematicBreak", indent(lines));
        }
        Ast::Table(Table {
            children, align, ..
        }) => {
            print!("{}Table align={align:?}", indent(lines));
            assert!(children.iter().all(|ast| matches!(ast, Ast::TableRow(_))));
            print_children(children, dir, lines);
        }
        Ast::TableRow(TableRow { children, .. }) => {
            print!("{}TableRow", indent(lines));
            assert!(children.iter().all(|ast| matches!(ast, Ast::TableCell(_))));
            print_children(children, dir, lines);
        }
        Ast::TableCell(TableCell { children, .. }) => {
            print!("{}TableCell", indent(lines));
            print_children(children, dir, lines);
        }
        Ast::Image(Image { url, alt, .. }) => {
            print!("{}Image: url={url} alt={alt}", indent(lines));
            if let Some(img) = ImageReader::open(dir.join(url))
                .ok()
                .and_then(|f| f.decode().ok())
            {
                print!(" size={}x{}", img.width(), img.height())
            };
        }

        Ast::Html(_)
        | Ast::Toml(_)
        | Ast::Yaml(_)
        | Ast::Math(_)
        | Ast::Break(_)
        | Ast::InlineMath(_)
        | Ast::LinkReference(_)
        | Ast::ImageReference(_)
        | Ast::MdxjsEsm(_)
        | Ast::MdxJsxFlowElement(_)
        | Ast::MdxTextExpression(_)
        | Ast::MdxFlowExpression(_)
        | Ast::MdxJsxTextElement(_)
        | Ast::Definition(_)
        | Ast::FootnoteReference(_)
        | Ast::FootnoteDefinition(_) => todo!(),
    }
}

fn print_children(children: Vec<Ast>, dir: &Path, lines: &mut Vec<bool>) {
    let last = children.len() - 1;
    for (i, ast) in children.into_iter().enumerate() {
        println!();
        lines.push(i < last);
        print_ast(ast, dir, lines);
        lines.pop();
    }
}

fn indent(lines: &[bool]) -> String {
    let mut ans = String::new();
    for &need in &lines[..lines.len() - 1] {
        ans.push(if need { ':' } else { ' ' });
        ans.push(' ');
    }
    ans
}
