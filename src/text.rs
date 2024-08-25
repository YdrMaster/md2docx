use crate::style::INLINE_CODE_STYLE_ID;
use docx_rs::Run;
use markdown::mdast::{self, Emphasis, Node as Ast, Strong};

pub enum Text {
    Normal(String),
    Strong(String),
    Emphasis(String),
    StrongEmphasis(String),
    InlineCode(String),
}

impl Text {
    pub fn strong(self) -> Self {
        match self {
            Self::Normal(text) => Self::Strong(text),
            Self::Strong(text) => Self::Strong(text),
            Self::Emphasis(text) => Self::StrongEmphasis(text),
            Self::StrongEmphasis(text) => Self::StrongEmphasis(text),
            Self::InlineCode(text) => Self::InlineCode(text),
        }
    }

    pub fn emphasis(self) -> Self {
        match self {
            Self::Normal(text) => Self::Emphasis(text),
            Self::Strong(text) => Self::StrongEmphasis(text),
            Self::Emphasis(text) => Self::Emphasis(text),
            Self::StrongEmphasis(text) => Self::StrongEmphasis(text),
            Self::InlineCode(text) => Self::InlineCode(text),
        }
    }

    pub fn into_run(self) -> Run {
        match self {
            Self::Normal(text) => Run::new().add_text(text),
            Self::Strong(text) => Run::new().add_text(text).bold(),
            Self::Emphasis(text) => Run::new().add_text(text).italic(),
            Self::StrongEmphasis(text) => Run::new().add_text(text).bold().italic(),
            Self::InlineCode(text) => Run::new().add_text(text).style(INLINE_CODE_STYLE_ID),
        }
    }
}

pub fn from_text(text: mdast::Text) -> Text {
    Text::Normal(text.value)
}

pub fn from_inline_code(inline_code: mdast::InlineCode) -> Text {
    Text::InlineCode(inline_code.value)
}

pub fn from_strong(strong: Strong) -> Vec<Text> {
    let mut ans = Vec::new();
    for node in strong.children {
        match node {
            Ast::Text(text) => ans.push(from_text(text).strong()),
            Ast::InlineCode(inline_code) => ans.push(from_inline_code(inline_code).strong()),
            Ast::Emphasis(emphasis) => {
                ans.extend(from_emphasis(emphasis).into_iter().map(|t| t.strong()))
            }

            Ast::BlockQuote(_) => todo!(),
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
            Ast::ImageReference(_) => todo!(),
            Ast::MdxJsxTextElement(_) => todo!(),
            Ast::Link(_) => todo!(),
            Ast::LinkReference(_) => todo!(),
            Ast::MdxFlowExpression(_) => todo!(),
            Ast::ListItem(_) => todo!(),
            Ast::Definition(_) => todo!(),
            Ast::Paragraph(_) => todo!(),

            Ast::Strong(_)
            | Ast::Root(_)
            | Ast::ThematicBreak(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Image(_)
            | Ast::Heading(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),
        }
    }
    ans
}

pub fn from_emphasis(strong: Emphasis) -> Vec<Text> {
    let mut ans = Vec::new();
    for node in strong.children {
        match node {
            Ast::Text(text) => ans.push(from_text(text).emphasis()),
            Ast::InlineCode(inline_code) => ans.push(from_inline_code(inline_code).emphasis()),
            Ast::Strong(strong) => {
                ans.extend(from_strong(strong).into_iter().map(|t| t.emphasis()))
            }

            Ast::BlockQuote(_) => todo!(),
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
            Ast::ImageReference(_) => todo!(),
            Ast::MdxJsxTextElement(_) => todo!(),
            Ast::Link(_) => todo!(),
            Ast::LinkReference(_) => todo!(),
            Ast::MdxFlowExpression(_) => todo!(),
            Ast::ListItem(_) => todo!(),
            Ast::Definition(_) => todo!(),
            Ast::Paragraph(_) => todo!(),

            Ast::Emphasis(_)
            | Ast::Root(_)
            | Ast::ThematicBreak(_)
            | Ast::Code(_)
            | Ast::Math(_)
            | Ast::Image(_)
            | Ast::Heading(_)
            | Ast::Table(_)
            | Ast::TableRow(_)
            | Ast::TableCell(_) => unreachable!(),
        }
    }
    ans
}
