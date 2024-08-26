use super::{docx, md, paragraph::from_link, style::inline_code_style, Ast};

pub fn to_paragraph_children(children: impl IntoIterator<Item = Ast>) -> Vec<docx::ParagraphChild> {
    type Child = docx::ParagraphChild;
    let mut ans = Vec::new();
    for ast in children {
        match ast {
            Ast::Text(text) => ans.push(Text::from(text).into_child()),
            Ast::InlineCode(inline_code) => ans.push(Text::from(inline_code).into_child()),
            Ast::Strong(strong) => {
                ans.extend(from_strong(strong).into_iter().map(|t| t.into_child()))
            }
            Ast::Emphasis(emphasis) => {
                ans.extend(from_emphasis(emphasis).into_iter().map(|t| t.into_child()))
            }
            Ast::Link(link) => ans.push(Child::Hyperlink(from_link(link))),

            Ast::Root(_)
            | Ast::Heading(_)
            | Ast::BlockQuote(_)
            | Ast::Code(_)
            | Ast::Math(_)
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
            | Ast::LinkReference(_)
            | Ast::MdxFlowExpression(_)
            | Ast::ThematicBreak(_)
            | Ast::Definition(_)
            | Ast::Paragraph(_) => todo!(),
        };
    }
    ans
}

pub struct Text {
    style: TextStyle,
    content: String,
}

enum TextStyle {
    Normal,
    Strong,
    Emphasis,
    StrongEmphasis,
    InlineCode,
}

impl Text {
    pub fn strong(self) -> Self {
        Self {
            style: match self.style {
                TextStyle::Normal => TextStyle::Strong,
                TextStyle::Strong => TextStyle::Strong,
                TextStyle::Emphasis => TextStyle::StrongEmphasis,
                TextStyle::StrongEmphasis => TextStyle::StrongEmphasis,
                TextStyle::InlineCode => TextStyle::InlineCode,
            },
            content: self.content,
        }
    }

    pub fn emphasis(self) -> Self {
        Self {
            style: match self.style {
                TextStyle::Normal => TextStyle::Emphasis,
                TextStyle::Strong => TextStyle::StrongEmphasis,
                TextStyle::Emphasis => TextStyle::Emphasis,
                TextStyle::StrongEmphasis => TextStyle::StrongEmphasis,
                TextStyle::InlineCode => TextStyle::InlineCode,
            },
            content: self.content,
        }
    }

    pub fn into_child(self) -> docx::ParagraphChild {
        let run = docx::Run::new().add_text(self.content);
        let run = match self.style {
            TextStyle::Normal => run,
            TextStyle::Strong => run.bold(),
            TextStyle::Emphasis => run.italic(),
            TextStyle::StrongEmphasis => run.bold().italic(),
            TextStyle::InlineCode => inline_code_style(run),
        };
        docx::ParagraphChild::Run(Box::new(run))
    }
}

impl From<md::Text> for Text {
    fn from(value: md::Text) -> Self {
        Self {
            style: TextStyle::Normal,
            content: value.value,
        }
    }
}

impl From<md::InlineCode> for Text {
    fn from(value: md::InlineCode) -> Self {
        Self {
            style: TextStyle::InlineCode,
            content: value.value,
        }
    }
}

pub fn from_strong(strong: md::Strong) -> Vec<Text> {
    let mut ans = Vec::new();
    for node in strong.children {
        match node {
            Ast::Text(text) => ans.push(Text::from(text).strong()),
            Ast::InlineCode(inline_code) => ans.push(Text::from(inline_code).strong()),
            Ast::Emphasis(emphasis) => {
                ans.extend(from_emphasis(emphasis).into_iter().map(|t| t.strong()))
            }

            Ast::Strong(_)
            | Ast::Root(_)
            | Ast::ThematicBreak(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Image(_)
            | Ast::ImageReference(_)
            | Ast::Heading(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),

            Ast::BlockQuote(_)
            | Ast::FootnoteDefinition(_)
            | Ast::MdxJsxFlowElement(_)
            | Ast::List(_)
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
            | Ast::ListItem(_)
            | Ast::Definition(_)
            | Ast::Paragraph(_) => todo!(),
        }
    }
    ans
}

pub fn from_emphasis(strong: md::Emphasis) -> Vec<Text> {
    let mut ans = Vec::new();
    for node in strong.children {
        match node {
            Ast::Text(text) => ans.push(Text::from(text).emphasis()),
            Ast::InlineCode(inline_code) => ans.push(Text::from(inline_code).emphasis()),
            Ast::Strong(strong) => {
                ans.extend(from_strong(strong).into_iter().map(|t| t.emphasis()))
            }

            Ast::Emphasis(_)
            | Ast::Root(_)
            | Ast::ThematicBreak(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Image(_)
            | Ast::ImageReference(_)
            | Ast::Heading(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),

            Ast::BlockQuote(_)
            | Ast::FootnoteDefinition(_)
            | Ast::MdxJsxFlowElement(_)
            | Ast::List(_)
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
            | Ast::ListItem(_)
            | Ast::Definition(_)
            | Ast::Paragraph(_) => todo!(),
        }
    }
    ans
}
