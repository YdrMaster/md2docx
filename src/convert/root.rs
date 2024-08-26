use super::{
    docx::Docx, list::from_list, md, paragraph::from_heading, paragraph::from_paragraph, Ast,
};

pub fn from_root(root: md::Root) -> Docx {
    let mut docx = Docx::new();
    for node in root.children {
        docx = match node {
            Ast::Heading(heading) => docx.add_paragraph(from_heading(heading)),
            Ast::Paragraph(paragraph) => docx.add_paragraph(from_paragraph(paragraph)),
            Ast::List(list) => from_list(list)
                .into_iter()
                .fold(docx, |docx, p| docx.add_paragraph(p)),

            Ast::Root(_) | Ast::ListItem(_) => unreachable!(),

            Ast::BlockQuote(_)
            | Ast::FootnoteDefinition(_)
            | Ast::MdxJsxFlowElement(_)
            | Ast::MdxjsEsm(_)
            | Ast::Toml(_)
            | Ast::Yaml(_)
            | Ast::Break(_)
            | Ast::InlineCode(_)
            | Ast::InlineMath(_)
            | Ast::Delete(_)
            | Ast::Emphasis(_)
            | Ast::MdxTextExpression(_)
            | Ast::FootnoteReference(_)
            | Ast::Html(_)
            | Ast::Image(_)
            | Ast::ImageReference(_)
            | Ast::MdxJsxTextElement(_)
            | Ast::Link(_)
            | Ast::LinkReference(_)
            | Ast::Strong(_)
            | Ast::Text(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::MdxFlowExpression(_)
            | Ast::Table(_)
            | Ast::ThematicBreak(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_)
            | Ast::Definition(_) => todo!(),
        }
    }
    docx
}
