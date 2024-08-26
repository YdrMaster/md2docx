use super::{docx, md, numbering::ListNumbering, paragraph::from_paragraph, Ast};

pub fn from_list(list: md::List) -> Vec<docx::Paragraph> {
    let md::List {
        children,
        ordered,
        start,
        ..
    } = list;
    if ordered {
        assert!(matches!(start, Some(1)));
    } else {
        assert!(matches!(start, None));
    }

    let numbering = ListNumbering::new(ordered);
    let mut ans = Vec::new();

    for ast in children {
        let Ast::ListItem(md::ListItem { children, .. }) = ast else {
            unreachable!()
        };
        let mut children = children.into_iter();
        let Some(Ast::Paragraph(first)) = children.next() else {
            unreachable!()
        };
        ans.push(numbering.apply(from_paragraph(first)));

        for ast in children {
            match ast {
                Ast::Paragraph(p) => ans.push(from_paragraph(p)),
                Ast::List(list) => ans.extend(from_list(list)),

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
                | Ast::Heading(_)
                | Ast::Table(_)
                | Ast::ThematicBreak(_)
                | Ast::TableRow(_)
                | Ast::TableCell(_)
                | Ast::Definition(_) => todo!(),
            }
        }
    }

    ans
}
