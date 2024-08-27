use super::{docx, md, style::inline_code_style, Ast};

pub fn to_paragraph_children(children: impl IntoIterator<Item = Ast>) -> Vec<docx::ParagraphChild> {
    children
        .into_iter()
        .flat_map(TextAndLink::from_ast)
        .map(TextAndLink::into_child)
        .collect()
}

struct Text {
    style: TextStyle,
    content: String,
}

enum TextStyle {
    Normal {
        strong: bool,
        emphasis: bool,
        delete: bool,
    },
    InlineCode,
}

impl Text {
    fn strong(&mut self) {
        if let TextStyle::Normal { strong, .. } = &mut self.style {
            *strong = true;
        }
    }

    fn emphasis(&mut self) {
        if let TextStyle::Normal { emphasis, .. } = &mut self.style {
            *emphasis = true;
        }
    }

    fn delete(&mut self) {
        if let TextStyle::Normal { delete, .. } = &mut self.style {
            *delete = true;
        }
    }

    fn into_run(self) -> docx::Run {
        let mut run = docx::Run::new().add_text(self.content);
        match self.style {
            TextStyle::Normal {
                strong,
                emphasis,
                delete,
            } => {
                if strong {
                    run.run_property = run.run_property.bold();
                }
                if emphasis {
                    run.run_property = run.run_property.italic();
                }
                if delete {
                    run.run_property = run.run_property.strike();
                }
                run
            }
            TextStyle::InlineCode => inline_code_style(run),
        }
    }
}

enum TextAndLink {
    Text(Text),
    Link { text: Vec<Text>, url: String },
}

impl TextAndLink {
    fn strong(mut self) -> Self {
        match &mut self {
            Self::Text(t) => t.strong(),
            Self::Link { text, .. } => {
                for t in text {
                    t.strong()
                }
            }
        }
        self
    }

    fn emphasis(mut self) -> Self {
        match &mut self {
            Self::Text(t) => t.emphasis(),
            Self::Link { text, .. } => {
                for t in text {
                    t.emphasis()
                }
            }
        }
        self
    }

    fn delete(mut self) -> Self {
        match &mut self {
            Self::Text(t) => t.delete(),
            Self::Link { text, .. } => {
                for t in text {
                    t.delete()
                }
            }
        }
        self
    }

    fn from_ast(ast: Ast) -> Vec<Self> {
        match ast {
            Ast::Text(md::Text { value, .. }) => vec![Self::Text(Text {
                style: TextStyle::Normal {
                    strong: false,
                    emphasis: false,
                    delete: false,
                },
                content: value,
            })],
            Ast::InlineCode(md::InlineCode { value, .. }) => vec![Self::Text(Text {
                style: TextStyle::InlineCode,
                content: value,
            })],
            Ast::Link(md::Link { children, url, .. }) => vec![Self::Link {
                text: children
                    .into_iter()
                    .flat_map(Self::from_ast)
                    .map(|it| {
                        let Self::Text(text) = it else { unreachable!() };
                        text
                    })
                    .collect(),
                url,
            }],

            Ast::Strong(md::Strong { children, .. }) => children
                .into_iter()
                .flat_map(Self::from_ast)
                .map(TextAndLink::strong)
                .collect(),
            Ast::Emphasis(md::Emphasis { children, .. }) => children
                .into_iter()
                .flat_map(Self::from_ast)
                .map(TextAndLink::emphasis)
                .collect(),
            Ast::Delete(md::Delete { children, .. }) => children
                .into_iter()
                .flat_map(Self::from_ast)
                .map(TextAndLink::delete)
                .collect(),

            _ => todo!(),
        }
    }

    fn into_child(self) -> docx::ParagraphChild {
        match self {
            Self::Text(text) => docx::ParagraphChild::Run(Box::new(text.into_run())),
            Self::Link { text, url } => {
                let mut hyperlink = docx::Hyperlink::new(url, docx_rs::HyperlinkType::External);
                for text in text {
                    hyperlink = hyperlink.add_run(text.into_run());
                }
                docx::ParagraphChild::Hyperlink(hyperlink)
            }
        }
    }
}
