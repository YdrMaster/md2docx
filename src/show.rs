use markdown::{
    mdast::{
        BlockQuote, Code, Delete, Emphasis, Heading, InlineCode, Link, List, ListItem, Node as Ast,
        Paragraph, Root, Strong, Table, TableCell, TableRow, Text, ThematicBreak,
    },
    to_mdast, ParseOptions,
};
use std::{fs::read_to_string, path::PathBuf};

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
        print_ast(Ast::Root(root), &mut lines);
        println!();
        println!();
    }
}

fn print_ast(ast: Ast, lines: &mut Vec<bool>) {
    match ast {
        Ast::Root(Root { children, .. }) => {
            print!("{}Root", indent(lines));
            print_children(children, lines);
        }
        Ast::Heading(Heading {
            children, depth, ..
        }) => {
            print!("{}Heading {depth}", indent(lines));
            print_children(children, lines);
        }
        Ast::Paragraph(Paragraph { children, .. }) => {
            print!("{}Paragraph", indent(lines));
            print_children(children, lines);
        }
        Ast::Link(Link { children, url, .. }) => {
            print!("{}Link: {url}", indent(lines));
            print_children(children, lines);
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
            print_children(children, lines);
        }
        Ast::ListItem(ListItem { children, .. }) => {
            print!("{}Item", indent(lines));
            print_children(children, lines);
        }
        Ast::Text(Text { value, .. }) => {
            print!("{}Text: {value:?}", indent(lines));
        }
        Ast::InlineCode(InlineCode { value, .. }) => {
            print!("{}InlineCode: {value:?}", indent(lines));
        }
        Ast::Delete(Delete { children, .. }) => {
            print!("{}Delete", indent(lines));
            print_children(children, lines);
        }
        Ast::Strong(Strong { children, .. }) => {
            print!("{}Strong", indent(lines));
            print_children(children, lines);
        }
        Ast::Emphasis(Emphasis { children, .. }) => {
            print!("{}Emphasis", indent(lines));
            print_children(children, lines);
        }

        Ast::BlockQuote(BlockQuote { children, .. }) => {
            print!("{}BlockQuote", indent(lines));
            print_children(children, lines);
        }
        Ast::Code(Code {
            value, lang, meta, ..
        }) => {
            print!(
                "{}Code(lang={lang:?} meta={meta:?}): {value:?}",
                indent(lines)
            );
        }
        Ast::ThematicBreak(ThematicBreak { .. }) => {
            print!("{}ThematicBreak", indent(lines));
        }
        Ast::Table(Table {
            children, align, ..
        }) => {
            print!("{}Table align={align:?}", indent(lines));
            assert!(children.iter().all(|ast| matches!(ast, Ast::TableRow(_))));
            print_children(children, lines);
        }
        Ast::TableRow(TableRow { children, .. }) => {
            print!("{}TableRow", indent(lines));
            assert!(children.iter().all(|ast| matches!(ast, Ast::TableCell(_))));
            print_children(children, lines);
        }
        Ast::TableCell(TableCell { children, .. }) => {
            print!("{}TableCell", indent(lines));
            print_children(children, lines);
        }

        Ast::FootnoteDefinition(_) => todo!(),
        Ast::MdxJsxFlowElement(_) => todo!(),
        Ast::MdxjsEsm(_) => todo!(),
        Ast::Toml(_) => todo!(),
        Ast::Yaml(_) => todo!(),
        Ast::Break(_) => todo!(),
        Ast::InlineMath(_) => todo!(),
        Ast::MdxTextExpression(_) => todo!(),
        Ast::FootnoteReference(_) => todo!(),
        Ast::Html(_) => todo!(),
        Ast::Image(_) => todo!(),
        Ast::ImageReference(_) => todo!(),
        Ast::MdxJsxTextElement(_) => todo!(),
        Ast::LinkReference(_) => todo!(),
        Ast::Math(_) => todo!(),
        Ast::MdxFlowExpression(_) => todo!(),
        Ast::Definition(_) => todo!(),
    }
}

fn print_children(children: Vec<Ast>, lines: &mut Vec<bool>) {
    let last = children.len() - 1;
    for (i, ast) in children.into_iter().enumerate() {
        println!();
        lines.push(i < last);
        print_ast(ast, lines);
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
