use super::{
    docx, md,
    style::{caption_style, code_style, table_style},
    Ast,
};

pub fn from_code(code: md::Code) -> (docx::Table, docx::Paragraph) {
    use docx::{Run, Table, TableCell, TableRow};

    #[inline(always)]
    fn paragraph_from(text: String) -> docx::Paragraph {
        docx::Paragraph::new().add_run(Run::new().add_text(text))
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

pub fn from_block_quote(quote: md::BlockQuote) -> docx::Table {
    use super::{list::from_list, paragraph::from_paragraph};
    use docx::{Table, TableCell, TableRow};

    let md::BlockQuote { children, .. } = quote;

    let mut block = TableCell::new();
    for ast in children {
        block = match ast {
            Ast::Paragraph(paragraph) => block.add_paragraph(from_paragraph(paragraph)),
            Ast::BlockQuote(quote) => block
                .add_table(from_block_quote(quote))
                .add_paragraph(docx::Paragraph::new()),
            Ast::Code(code) => {
                let (code, caption) = from_code(code);
                block.add_table(code).add_paragraph(caption)
            }
            Ast::List(list) => from_list(list)
                .into_iter()
                .fold(block, |block, p| block.add_paragraph(p)),

            Ast::Root(_) | Ast::Heading(_) => unreachable!(),

            _ => todo!(),
        };
    }
    table_style(Table::new(vec![TableRow::new(vec![block])]))
}
