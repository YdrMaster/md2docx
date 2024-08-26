use docx_rs::{Docx, Paragraph, Run, Style, StyleType};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crate::heading::max_heading_depth;

const HEADING_DTYLE_ID: fn(u8) -> String = |depth| format!("Heading{depth}");
const INLINE_CODE_STYLE_ID: &str = "InlineCode";

static INLINE_CODE_STYLE: AtomicBool = AtomicBool::new(false);

pub fn heading_style(p: Paragraph, depth: u8) -> Paragraph {
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
