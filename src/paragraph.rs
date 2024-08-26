use crate::text::{from_emphasis, from_strong, Text};
use docx_rs::Paragraph;
use markdown::mdast::{self, Node as Ast};

pub fn from_paragraph(paragraph: mdast::Paragraph) -> Paragraph {
    let mut p = Paragraph::new();
    for node in paragraph.children {
        p = match node {
            Ast::Text(text) => p.add_run(Text::from(text).into_run()),
            Ast::InlineCode(inline_code) => p.add_run(Text::from(inline_code).into_run()),
            Ast::Strong(strong) => from_strong(strong)
                .into_iter()
                .fold(p, |p, text| p.add_run(text.into_run())),
            Ast::Emphasis(emphasis) => from_emphasis(emphasis)
                .into_iter()
                .fold(p, |p, text| p.add_run(text.into_run())),

            Ast::Paragraph(_)
            | Ast::Root(_)
            | Ast::BlockQuote(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Heading(_)
            | Ast::ThematicBreak(_)
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
            | Ast::Image(_)
            | Ast::ImageReference(_)
            | Ast::MdxJsxTextElement(_)
            | Ast::Link(_)
            | Ast::LinkReference(_)
            | Ast::MdxFlowExpression(_)
            | Ast::Definition(_) => todo!(),
        }
    }
    p
}
