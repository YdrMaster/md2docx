use super::{
    docx::{self, Docx},
    list::from_list,
    md,
    paragraph::{from_heading, from_paragraph},
    table::{from_block_quote, from_code},
    Ast,
};

pub fn from_root(root: md::Root) -> Docx {
    let mut docx = Docx::new();
    for node in root.children {
        docx = match node {
            Ast::Heading(heading) => docx.add_paragraph(from_heading(heading)),
            Ast::Paragraph(paragraph) => docx.add_paragraph(from_paragraph(paragraph)),
            Ast::Code(code) => {
                let (code, caption) = from_code(code);
                docx.add_table(code).add_paragraph(caption)
            }
            Ast::BlockQuote(quote) => docx
                .add_table(from_block_quote(quote))
                .add_paragraph(docx::Paragraph::new()),
            Ast::List(list) => from_list(list)
                .into_iter()
                .fold(docx, |docx, p| docx.add_paragraph(p)),

            Ast::Table(_) | Ast::ThematicBreak(_) => todo!(),

            Ast::Root(_)
            | Ast::Text(_)
            | Ast::InlineCode(_)
            | Ast::Strong(_)
            | Ast::Emphasis(_)
            | Ast::Delete(_)
            | Ast::Image(_)
            | Ast::ImageReference(_)
            | Ast::Link(_)
            | Ast::LinkReference(_)
            | Ast::ListItem(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),

            Ast::Html(_)
            | Ast::Toml(_)
            | Ast::Yaml(_)
            | Ast::Math(_)
            | Ast::Break(_)
            | Ast::InlineMath(_)
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
    docx
}
