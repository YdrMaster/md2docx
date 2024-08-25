use crate::{
    style::heading_style_id,
    text::{from_emphasis, from_inline_code, from_strong, from_text},
    BuildError,
};
use docx_rs::Paragraph;
use markdown::mdast::{Heading, Node as Ast};

pub fn from_heading(heading: Heading) -> Result<Paragraph, BuildError> {
    let mut p = Paragraph::new();
    for node in heading.children {
        p = match node {
            Ast::Text(text) => p.add_run(from_text(text).into_run()),
            Ast::InlineCode(inline_code) => p.add_run(from_inline_code(inline_code).into_run()),
            Ast::Strong(strong) => from_strong(strong)
                .into_iter()
                .fold(p, |p, text| p.add_run(text.into_run())),
            Ast::Emphasis(emphasis) => from_emphasis(emphasis)
                .into_iter()
                .fold(p, |p, text| p.add_run(text.into_run())),

            Ast::BlockQuote(_) => todo!(),
            Ast::FootnoteDefinition(_) => todo!(),
            Ast::MdxJsxFlowElement(_) => todo!(),
            Ast::List(_) => todo!(),
            Ast::MdxjsEsm(_) => todo!(),
            Ast::Toml(_) => todo!(),
            Ast::Yaml(_) => todo!(),
            Ast::Break(_) => todo!(),
            Ast::InlineMath(_) => todo!(),
            Ast::Delete(_) => todo!(),
            Ast::MdxTextExpression(_) => todo!(),
            Ast::FootnoteReference(_) => todo!(),
            Ast::Html(_) => todo!(),
            Ast::ImageReference(_) => todo!(),
            Ast::MdxJsxTextElement(_) => todo!(),
            Ast::Link(_) => todo!(),
            Ast::LinkReference(_) => todo!(),
            Ast::MdxFlowExpression(_) => todo!(),
            Ast::ListItem(_) => todo!(),
            Ast::Definition(_) => todo!(),
            Ast::Paragraph(_) => todo!(),

            Ast::Heading(_)
            | Ast::Root(_)
            | Ast::ThematicBreak(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Image(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),
        };
    }
    Ok(p.style(&heading_style_id(heading.depth)))
}
