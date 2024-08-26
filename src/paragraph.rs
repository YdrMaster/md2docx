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
            Ast::Image(_) => todo!(),
            Ast::ImageReference(_) => todo!(),
            Ast::MdxJsxTextElement(_) => todo!(),
            Ast::Link(_) => todo!(),
            Ast::LinkReference(_) => todo!(),
            Ast::MdxFlowExpression(_) => todo!(),
            Ast::ListItem(_) => todo!(),
            Ast::Definition(_) => todo!(),

            Ast::Paragraph(_)
            | Ast::Root(_)
            | Ast::BlockQuote(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Heading(_)
            | Ast::ThematicBreak(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),
        }
    }
    p
}
