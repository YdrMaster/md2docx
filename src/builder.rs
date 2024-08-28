use docx_rs::{Docx, IndentLevel, NumberingId, Paragraph, Table};

pub struct DocxBuilder {
    elements: Vec<DocxElement>,
    max_heading_level: Option<usize>,
}

enum DocxElement {
    Paragraph(Paragraph),
    Table(Table),
}

impl DocxBuilder {
    pub fn new() -> Self {
        DocxBuilder {
            elements: Vec::new(),
            max_heading_level: None,
        }
    }

    pub fn push_heading(&mut self, level: usize, mut p: Paragraph) {
        assert!(
            p.property.style.is_none(),
            "Heading paragraph should not have customed style"
        );
        assert!(
            p.property.numbering_property.is_none(),
            "Heading paragraph should not have customed numbering"
        );

        self.max_heading_level = Some(self.max_heading_level.map_or(level, |max| max.max(level)));
        if let Some(level) = level.checked_sub(1) {
            p = p.numbering(NumberingId::new(HEADING_NUMBERING), IndentLevel::new(level));
        }

        self.push_paragraph(p.style(&HEADING_STYLE_ID(level + 1)));
    }

    #[inline]
    pub fn push_paragraph(&mut self, p: Paragraph) {
        self.elements.push(DocxElement::Paragraph(p));
    }

    #[inline]
    pub fn push_table(&mut self, t: Table) {
        self.elements.push(DocxElement::Table(t));
    }

    pub fn build(self) -> Docx {
        let mut docx = Docx::new();
        for element in self.elements {
            docx = match element {
                DocxElement::Paragraph(p) => docx.add_paragraph(p),
                DocxElement::Table(t) => docx.add_table(t),
            };
        }
        docx
    }
}

const HEADING_STYLE_ID: fn(usize) -> String = |level| format!("Heading{level}");
const HEADING_NUMBERING: usize = 1;
