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
use toml::Value as Val;

const HEADING_STYLE_ID: fn(u8) -> String = |depth| format!("Heading{depth}");
const BASE_CODE_STYLE_ID: &str = "Code";
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

pub fn add_style(mut docx: Docx, settings: toml::Table) -> Docx {
    use docx::{
        AlignmentType::{Center, Left},
        Style, StyleType, TableAlignmentType,
    };

    docx = docx.add_style(modify(
        Style::new(BODY_TEXT_STYLE_ID, StyleType::Paragraph).name("Body Text"),
        &settings,
    ));
    for i in 1..=max_heading_depth() {
        docx = docx.add_style(modify(
            Style::new(HEADING_STYLE_ID(i as _), StyleType::Paragraph)
                .based_on(BODY_TEXT_STYLE_ID)
                .name(format!("Heading {i}")),
            &settings,
        ));
    }
    if INLINE_CODE_STYLE.load(Relaxed) {
        docx = docx.add_style(modify(
            Style::new(INLINE_CODE_STYLE_ID, StyleType::Character).name("Inline Code"),
            &settings,
        ));
    }
    if IMAGE_STYLE.load(Relaxed) {
        docx = docx.add_style(modify(
            Style::new(IMAGE_STYLE_ID, StyleType::Paragraph)
                .align(Center)
                .name(IMAGE_STYLE_ID),
            &settings,
        ));
    }
    if TABLE_STYLE.load(Relaxed) {
        docx = docx.add_style(modify(
            Style::new(TABLE_STYLE_ID, StyleType::Table)
                .based_on(BODY_TEXT_STYLE_ID)
                .table_align(TableAlignmentType::Center)
                .name(TABLE_STYLE_ID),
            &settings,
        ));
    }
    if CAPTION_STYLE.load(Relaxed) {
        docx = docx.add_style(modify(
            Style::new(CAPTION_STYLE_ID, StyleType::Paragraph)
                .align(Center)
                .name(CAPTION_STYLE_ID),
            &settings,
        ));
    }
    {
        let languages = LANGUAGES.lock().unwrap();
        if !languages.is_empty() {
            docx = docx.add_style(modify(
                Style::new(BASE_CODE_STYLE_ID, StyleType::Paragraph)
                    .align(Left)
                    .name("Code"),
                &settings,
            ));
        }
        for lang in &*languages {
            docx = docx.add_style(modify(
                Style::new(CODE_STYLE_ID(lang), StyleType::Paragraph)
                    .based_on(BASE_CODE_STYLE_ID)
                    .name(format!("Code {lang}")),
                &settings,
            ));
        }
    }

    docx
}

fn modify(mut style: docx::Style, settings: &toml::Table) -> docx::Style {
    let Some(value) = settings.get(&style.style_id) else {
        return style;
    };
    let Val::Table(settings) = value else {
        panic!(
            "Invalid style settings, must be a table: {}",
            style.style_id
        )
    };
    for (key, val) in settings {
        match key.as_str() {
            "font" => style = set_fonts(style, val),
            "font-size" => style = set_font_size(style, val),
            "align" => style = set_alignment(style, val),
            key => eprintln!("Unknown style setting: {}", key),
        }
    }
    style
}

fn set_fonts(style: docx::Style, val: &Val) -> docx::Style {
    use docx::RunFonts;
    match val {
        Val::Array(arr) => match &arr[..] {
            [Val::String(font)] => style.fonts(RunFonts::new().east_asia(font).ascii(font)),
            [Val::String(asia), Val::String(ascii)] => {
                style.fonts(RunFonts::new().east_asia(asia).ascii(ascii))
            }
            [..] => panic!("Font array must be [font] or [asia, ascii]"),
        },
        Val::String(font) => style.fonts(RunFonts::new().east_asia(font).ascii(font)),
        _ => panic!("Font must be an array or a string"),
    }
}

fn set_font_size(style: docx::Style, val: &Val) -> docx::Style {
    #[inline(always)]
    fn map_size(size: f64) -> usize {
        (size * 2.) as _
    }

    match val {
        Val::String(size) => style.size(map_size(match size.as_str() {
            "初号" => 42.,
            "小初" => 36.,
            "一号" => 26.,
            "小一" => 24.,
            "二号" => 22.,
            "小二" => 18.,
            "三号" => 16.,
            "小三" => 15.,
            "四号" => 14.,
            "小四" => 12.,
            "五号" => 10.5,
            "小五" => 9.,
            "六号" => 7.5,
            "七号" => 5.5,
            "八号" => 5.,
            val => val
                .parse()
                .unwrap_or_else(|_| panic!("Invalid font size: {val}")),
        })),
        &Val::Integer(size) => style.size(map_size(size as _)),
        &Val::Float(size) => style.size(map_size(size)),
        _ => panic!("Font-size must be a string or a number"),
    }
}

fn set_alignment(style: docx::Style, val: &Val) -> docx::Style {
    use docx::{AlignmentType as P, StyleType, TableAlignmentType as T};
    match val {
        Val::String(align) => match align.as_str() {
            "center" | "c" => match style.style_type {
                StyleType::Paragraph => style.align(P::Center),
                StyleType::Table => style.table_align(T::Center),
                _ => panic!("alignment only supported for paragraphs and tables"),
            },
            "left" | "l" => match style.style_type {
                StyleType::Paragraph => style.align(P::Left),
                StyleType::Table => style.table_align(T::Left),
                _ => panic!("alignment only supported for paragraphs and tables"),
            },
            "right" | "r" => match style.style_type {
                StyleType::Paragraph => style.align(P::Right),
                StyleType::Table => style.table_align(T::Right),
                _ => panic!("alignment only supported for paragraphs and tables"),
            },
            _ => panic!("Invalid alignment: {align}"),
        },
        _ => panic!("align must be a string"),
    }
}
