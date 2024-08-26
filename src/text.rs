use crate::style::inline_code_style;
use docx_rs::Run;
use markdown::mdast::{self, Emphasis, Node as Ast, Strong};

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

    pub fn into_run(self) -> Run {
        let run = Run::new().add_text(self.content);
        match self.style {
            TextStyle::Normal => run,
            TextStyle::Strong => run.bold(),
            TextStyle::Emphasis => run.italic(),
            TextStyle::StrongEmphasis => run.bold().italic(),
            TextStyle::InlineCode => inline_code_style(run),
        }
    }
}

impl From<mdast::Text> for Text {
    fn from(value: mdast::Text) -> Self {
        Self {
            style: TextStyle::Normal,
            content: value.value,
        }
    }
}

impl From<mdast::InlineCode> for Text {
    fn from(value: mdast::InlineCode) -> Self {
        Self {
            style: TextStyle::InlineCode,
            content: value.value,
        }
    }
}

pub fn from_strong(strong: Strong) -> Vec<Text> {
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

pub fn from_emphasis(strong: Emphasis) -> Vec<Text> {
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
