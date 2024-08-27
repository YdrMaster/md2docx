use super::{
    docx::{self, Docx, Run},
    paragraph::max_heading_depth,
};
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        LazyLock, Mutex,
    },
};

const HEADING_STYLE_ID: fn(u8) -> String = |depth| format!("Heading{depth}");
const CODE_STYLE_ID: fn(&str) -> String = |lang| format!("Code-{}", lang.to_ascii_lowercase());
const INLINE_CODE_STYLE_ID: &str = "InlineCode";
const BODY_TEXT_STYLE_ID: &str = "BodyText";
const IMAGE_STYLE_ID: &str = "Image";
const TABLE_STYLE_ID: &str = "Table";
const CAPTION_STYLE_ID: &str = "Caption";

static LANGUAGES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Default::default());
static INLINE_CODE_STYLE: AtomicBool = AtomicBool::new(false);
static IMAGE_STYLE: AtomicBool = AtomicBool::new(false);
static TABLE_STYLE: AtomicBool = AtomicBool::new(false);
static CAPTION_STYLE: AtomicBool = AtomicBool::new(false);

pub fn heading_style(p: docx::Paragraph, depth: u8) -> docx::Paragraph {
    p.style(&HEADING_STYLE_ID(depth))
}

pub fn body_text_style(p: docx::Paragraph) -> docx::Paragraph {
    p.style(BODY_TEXT_STYLE_ID)
}

pub fn inline_code_style(run: Run) -> Run {
    INLINE_CODE_STYLE.store(true, Relaxed);
    run.style(INLINE_CODE_STYLE_ID)
}

pub fn code_style(p: docx::Paragraph, lang: &str) -> docx::Paragraph {
    LANGUAGES.lock().unwrap().insert(lang.into());
    p.style(&CODE_STYLE_ID(lang))
}

pub fn caption_style(p: docx::Paragraph) -> docx::Paragraph {
    CAPTION_STYLE.store(true, Relaxed);
    p.style(CAPTION_STYLE_ID)
}

pub fn image_style(p: docx::Paragraph) -> docx::Paragraph {
    IMAGE_STYLE.store(true, Relaxed);
    p.style(IMAGE_STYLE_ID)
}

pub fn table_style(t: docx::Table) -> docx::Table {
    TABLE_STYLE.store(true, Relaxed);
    t.style(TABLE_STYLE_ID)
}

pub fn add_style(mut docx: Docx) -> Docx {
    use docx::{
        AlignmentType::{Center, Left},
        Style, StyleType, TableAlignmentType,
    };

    for i in 1..=max_heading_depth() {
        docx = docx.add_style(
            Style::new(HEADING_STYLE_ID(i as _), StyleType::Paragraph).name(format!("Heading {i}")),
        );
    }
    if INLINE_CODE_STYLE.load(Relaxed) {
        docx = docx
            .add_style(Style::new(INLINE_CODE_STYLE_ID, StyleType::Character).name("Inline Code"));
    }
    if IMAGE_STYLE.load(Relaxed) {
        docx = docx.add_style(
            Style::new(IMAGE_STYLE_ID, StyleType::Paragraph)
                .align(Center)
                .name(IMAGE_STYLE_ID),
        );
    }
    if TABLE_STYLE.load(Relaxed) {
        docx = docx.add_style(
            Style::new(TABLE_STYLE_ID, StyleType::Table)
                .table_align(TableAlignmentType::Center)
                .name(TABLE_STYLE_ID),
        );
    }
    if CAPTION_STYLE.load(Relaxed) {
        docx = docx.add_style(
            Style::new(CAPTION_STYLE_ID, StyleType::Paragraph)
                .align(Center)
                .name(CAPTION_STYLE_ID),
        );
    }
    for lang in LANGUAGES.lock().unwrap().iter() {
        docx = docx.add_style(
            Style::new(CODE_STYLE_ID(lang), StyleType::Paragraph)
                .align(Left)
                .name(format!("Code {lang}")),
        );
    }
    docx
}
