use markdown::{
    mdast::{
        Emphasis, Heading, InlineCode, Link, List, ListItem, Node as Ast, Paragraph, Root, Strong,
        Text,
    },
    to_mdast, ParseOptions,
};
use std::{fmt, fs::read_to_string, path::PathBuf};

#[derive(Args, Default)]
pub struct Args {
    /// The file to show
    file: PathBuf,
}

impl Args {
    pub fn show(self) {
        let path = self.file;
        let md = read_to_string(&path).unwrap();
        let Ok(Ast::Root(Root { children, .. })) = to_mdast(&md, &ParseOptions::gfm()) else {
            panic!("Failed to parse markdown");
        };

        for ast in &children {
            println!("{}", ShowAst { ast, level: 0 })
        }
    }
}

struct ShowAst<'a> {
    ast: &'a Ast,
    level: usize,
}

impl fmt::Display for ShowAst<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.ast {
            Ast::Root(_) => unreachable!(),

            Ast::Heading(Heading {
                children, depth, ..
            }) => {
                write!(f, "{0:1$}- Heading {depth}", "", self.level * 2)?;
                fmt_children(f, children, self.level + 1)?;
            }
            Ast::Paragraph(Paragraph { children, .. }) => {
                write!(f, "{0:1$}- Paragraph", "", self.level * 2)?;
                fmt_children(f, children, self.level + 1)?;
            }
            Ast::Link(Link { children, url, .. }) => {
                write!(f, "{0:1$}- Link: {url}", "", self.level * 2)?;
                fmt_children(f, children, self.level + 1)?;
            }
            Ast::List(List {
                children,
                ordered,
                start,
                ..
            }) => {
                assert_eq!(start.is_some(), *ordered);
                if *ordered {
                    let Some(start) = start else { panic!() };
                    write!(f, "{0:1$}- List {start}..", "", self.level * 2)?;
                } else {
                    assert!(start.is_none());
                    write!(f, "{0:1$}- List", "", self.level * 2)?;
                }
                let level = self.level + 1;
                for ast in children {
                    write!(f, "{0:1$}---", "", level * 2)?;
                    let Ast::ListItem(ListItem { children, .. }) = ast else {
                        panic!()
                    };
                    fmt_children(f, children, level)?;
                }
            }
            Ast::Text(Text { value, .. }) => {
                write!(f, "{0:1$}- Text: {value:?}", "", self.level * 2)?;
            }
            Ast::InlineCode(InlineCode { value, .. }) => {
                write!(f, "{0:1$}- InlineCode: {value:?}", "", self.level * 2)?;
            }
            Ast::Strong(Strong { children, .. }) => {
                write!(f, "{0:1$}- Strong", "", self.level * 2)?;
                fmt_children(f, children, self.level + 1)?;
            }
            Ast::Emphasis(Emphasis { children, .. }) => {
                write!(f, "{0:1$}- Emphasis", "", self.level * 2)?;
                fmt_children(f, children, self.level + 1)?;
            }

            Ast::BlockQuote(_) => todo!(),
            Ast::FootnoteDefinition(_) => todo!(),
            Ast::MdxJsxFlowElement(_) => todo!(),
            Ast::MdxjsEsm(_) => todo!(),
            Ast::Toml(_) => todo!(),
            Ast::Yaml(_) => todo!(),
            Ast::Break(_) => todo!(),
            Ast::InlineMath(_) => todo!(),
            Ast::Delete(_) => todo!(),
            Ast::MdxTextExpression(_) => todo!(),
            Ast::FootnoteReference(_) => todo!(),
            Ast::Html(_) => todo!(),
            Ast::Image(_) => todo!(),
            Ast::ImageReference(_) => todo!(),
            Ast::MdxJsxTextElement(_) => todo!(),
            Ast::LinkReference(_) => todo!(),
            Ast::Code(_) => todo!(),
            Ast::Math(_) => todo!(),
            Ast::MdxFlowExpression(_) => todo!(),
            Ast::Table(_) => todo!(),
            Ast::ThematicBreak(_) => todo!(),
            Ast::TableRow(_) => todo!(),
            Ast::TableCell(_) => todo!(),
            Ast::ListItem(_) => todo!(),
            Ast::Definition(_) => todo!(),
        }
        Ok(())
    }
}

fn fmt_children(f: &mut fmt::Formatter, children: &[Ast], level: usize) -> fmt::Result {
    for ast in children {
        writeln!(f)?;
        write!(f, "{}", ShowAst { ast, level })?;
    }
    Ok(())
}
