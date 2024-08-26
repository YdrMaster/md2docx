use crate::{
    numbering::heading_numbering,
    style::heading_style,
    text::{from_emphasis, from_strong, Text},
};
use docx_rs::Paragraph;
use markdown::mdast::{Heading, Node as Ast};
use std::sync::atomic::{AtomicU8, Ordering::Relaxed};

static MAX_HEADING_DEPTH: AtomicU8 = AtomicU8::new(0);

pub fn from_heading(heading: Heading) -> Paragraph {
    let Heading {
        depth, children, ..
    } = heading;
    MAX_HEADING_DEPTH.fetch_max(depth, Relaxed);

    let mut p = Paragraph::new();
    for node in children {
        p = match node {
            Ast::Text(text) => p.add_run(Text::from(text).into_run()),
            Ast::InlineCode(inline_code) => p.add_run(Text::from(inline_code).into_run()),
            Ast::Strong(strong) => from_strong(strong)
                .into_iter()
                .fold(p, |p, text| p.add_run(text.into_run())),
            Ast::Emphasis(emphasis) => from_emphasis(emphasis)
                .into_iter()
                .fold(p, |p, text| p.add_run(text.into_run())),

            Ast::Heading(_)
            | Ast::Root(_)
            | Ast::Paragraph(_)
            | Ast::BlockQuote(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::ThematicBreak(_)
            | Ast::Image(_)
            | Ast::ImageReference(_)
            | Ast::List(_)
            | Ast::ListItem(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),

            Ast::FootnoteDefinition(_)
            | Ast::MdxJsxFlowElement(_)
            | Ast::MdxjsEsm(_)
            | Ast::Toml(_)
            | Ast::Yaml(_)
            | Ast::Break(_)
            | Ast::InlineMath(_)
            | Ast::Delete(_)
            | Ast::MdxTextExpression(_)
            | Ast::FootnoteReference(_)
            | Ast::Html(_)
            | Ast::MdxJsxTextElement(_)
            | Ast::Link(_)
            | Ast::LinkReference(_)
            | Ast::MdxFlowExpression(_)
            | Ast::Definition(_) => todo!(),
        };
    }
    p = heading_style(p, depth);
    p = heading_numbering(p, depth);
    p
}

pub fn max_heading_depth() -> usize {
    MAX_HEADING_DEPTH.load(Relaxed) as _
}
