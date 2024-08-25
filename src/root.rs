use crate::{heading::from_heading, paragraph::from_paragraph, BuildError};
use docx_rs::Docx;
use markdown::mdast::{Node as Ast, Root};

pub fn from_root(root: Root) -> Result<Docx, BuildError> {
    let mut docx = Docx::new();
    for node in root.children {
        docx = match node {
            Ast::Heading(heading) => docx.add_paragraph(from_heading(heading)?),
            Ast::Paragraph(paragraph) => docx.add_paragraph(from_paragraph(paragraph)?),

            Ast::BlockQuote(_) => todo!(),
            Ast::FootnoteDefinition(_) => todo!(),
            Ast::MdxJsxFlowElement(_) => todo!(),
            Ast::List(_) => todo!(),
            Ast::MdxjsEsm(_) => todo!(),
            Ast::Toml(_) => todo!(),
            Ast::Yaml(_) => todo!(),
            Ast::Break(_) => todo!(),
            Ast::InlineCode(_) => todo!(),
            Ast::InlineMath(_) => todo!(),
            Ast::Delete(_) => todo!(),
            Ast::Emphasis(_) => todo!(),
            Ast::MdxTextExpression(_) => todo!(),
            Ast::FootnoteReference(_) => todo!(),
            Ast::Html(_) => todo!(),
            Ast::Image(_) => todo!(),
            Ast::ImageReference(_) => todo!(),
            Ast::MdxJsxTextElement(_) => todo!(),
            Ast::Link(_) => todo!(),
            Ast::LinkReference(_) => todo!(),
            Ast::Strong(_) => todo!(),
            Ast::Text(_) => todo!(),
            Ast::Code(_) => todo!(),
            Ast::Math(_) => todo!(),
            Ast::MdxFlowExpression(_) => todo!(),
            Ast::Table(_) => todo!(),
            Ast::ThematicBreak(_) => todo!(),
            Ast::TableRow(_) => todo!(),
            Ast::TableCell(_) => todo!(),
            Ast::ListItem(_) => todo!(),
            Ast::Definition(_) => todo!(),

            Ast::Root(_) => unreachable!(),
        }
    }
    Ok(docx)
}
