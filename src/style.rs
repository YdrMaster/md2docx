use crate::{
    docx::{self, Docx, Run, Style, StyleType},
    paragraph::max_heading_depth,
};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

const HEADING_DTYLE_ID: fn(u8) -> String = |depth| format!("Heading{depth}");
const INLINE_CODE_STYLE_ID: &str = "InlineCode";

static INLINE_CODE_STYLE: AtomicBool = AtomicBool::new(false);

pub fn heading_style(p: docx::Paragraph, depth: u8) -> docx::Paragraph {
    p.style(&HEADING_DTYLE_ID(depth))
}

pub fn inline_code_style(run: Run) -> Run {
    INLINE_CODE_STYLE.store(true, Relaxed);
    run.style(INLINE_CODE_STYLE_ID)
}

pub fn add_style(mut docx: Docx) -> Docx {
    for i in 1..=max_heading_depth() {
        docx = docx.add_style(
            Style::new(HEADING_DTYLE_ID(i as _), StyleType::Paragraph).name(format!("Heading {i}")),
        );
    }
    if INLINE_CODE_STYLE.load(Relaxed) {
        docx = docx
            .add_style(Style::new(INLINE_CODE_STYLE_ID, StyleType::Character).name("Inline Code"));
    }
    docx
}
