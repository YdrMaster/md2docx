use super::{
    docx, md,
    style::{caption_style, code_style, table_style},
    Ast,
};
use std::path::Path;

pub fn from_code(code: md::Code) -> (docx::Table, docx::Paragraph) {
    use docx::{Table, TableCell, TableRow};

    #[inline(always)]
    fn paragraph_from(text: String) -> docx::Paragraph {
        docx::Paragraph::new().add_run(docx::Run::new().add_text(text))
    }

    let md::Code {
        value, lang, meta, ..
    } = code;

    let lang = lang.unwrap_or_default();
    let mut code = TableCell::new();
    for line in value.lines() {
        code = code.add_paragraph(code_style(paragraph_from(line.to_string()), &lang));
    }

    (
        table_style(Table::new(vec![TableRow::new(vec![code])])),
        caption_style(paragraph_from(meta.unwrap_or_default())),
    )
}

pub fn from_block_quote(quote: md::BlockQuote, dir: &Path) -> docx::Table {
    use docx::{Table, TableRow};
    let md::BlockQuote { children, .. } = quote;
    table_style(Table::new(vec![TableRow::new(vec![cell_from(
        children, dir,
    )])]))
}

pub fn from_table(table: md::Table) -> docx::Table {
    use super::text::to_paragraph_children;
    use docx::AlignmentType;
    use md::AlignKind;

    let md::Table {
        children, align, ..
    } = table;

    let table = docx::Table::new(
        children
            .into_iter()
            .map(|ast| {
                let Ast::TableRow(md::TableRow { children, .. }) = ast else {
                    unreachable!()
                };
                assert_eq!(children.len(), align.len());
                docx::TableRow::new(
                    children
                        .into_iter()
                        .zip(&align)
                        .map(|(ast, align)| {
                            let Ast::TableCell(md::TableCell { children, .. }) = ast else {
                                unreachable!()
                            };
                            let mut p = docx::Paragraph::new();
                            p.children.extend(to_paragraph_children(children));
                            docx::TableCell::new().add_paragraph(p.align(match align {
                                AlignKind::None => AlignmentType::Both,
                                AlignKind::Left => AlignmentType::Left,
                                AlignKind::Right => AlignmentType::Right,
                                AlignKind::Center => AlignmentType::Center,
                            }))
                        })
                        .collect(),
                )
            })
            .collect(),
    );

    table_style(table)
}

fn cell_from(children: impl IntoIterator<Item = Ast>, dir: &Path) -> docx::TableCell {
    use super::{list::from_list, paragraph::from_paragraph};

    let mut cell = docx::TableCell::new();
    for ast in children {
        cell = match ast {
            Ast::Paragraph(paragraph) => {
                let (paragraph, caption) = from_paragraph(paragraph, dir);
                match caption {
                    Some(c) => cell.add_paragraph(paragraph).add_paragraph(c),
                    None => cell.add_paragraph(paragraph),
                }
            }
            Ast::Code(code) => {
                let (code, caption) = from_code(code);
                cell.add_table(code).add_paragraph(caption)
            }
            Ast::BlockQuote(quote) => cell
                .add_table(from_block_quote(quote, dir))
                .add_paragraph(docx::Paragraph::new()),
            Ast::Table(table) => cell.add_table(from_table(table)),
            Ast::List(list) => from_list(list, dir)
                .into_iter()
                .fold(cell, |cell, p| cell.add_paragraph(p)),

            Ast::Root(_) | Ast::Heading(_) => unreachable!(),

            _ => todo!(),
        };
    }
    cell
}
