use crate::{
    docx::{self, Docx, IndentLevel, NumberingId},
    paragraph::max_heading_depth,
};
use std::sync::{LazyLock, Mutex};

const HEADING_NUMBERING: usize = 1;
const UNORDERED_NUMBERING: usize = 2;
static LIST_INFO: LazyLock<Mutex<Vec<bool>>> = LazyLock::new(|| Default::default());

/// 为标题添加多级列表编号，`depth` in 1..=6
pub fn heading_numbering(p: docx::Paragraph, depth: u8) -> docx::Paragraph {
    if let Some(depth) = depth.checked_sub(2) {
        p.numbering(
            NumberingId::new(HEADING_NUMBERING),
            IndentLevel::new(depth as _),
        )
    } else {
        p
    }
}

pub struct ListNumbering(usize);

impl ListNumbering {
    pub fn new(ordered: bool) -> Self {
        let mut list_info = LIST_INFO.lock().unwrap();
        let id = UNORDERED_NUMBERING + list_info.len();
        list_info.push(ordered);
        Self(id)
    }

    pub fn apply(&self, p: docx::Paragraph) -> docx::Paragraph {
        p.numbering(NumberingId::new(self.0), IndentLevel::new(0))
    }
}

pub fn add_numbering(docx: Docx) -> Docx {
    use docx::{
        AbstractNumbering, Level, LevelJc, LevelSuffixType::Space, LevelText, NumberFormat,
        Numbering, Start,
    };

    let mut heading = AbstractNumbering::new(HEADING_NUMBERING);
    for i in 1..max_heading_depth() {
        let level_text = (1..=i).fold(String::new(), |acc, i| format!("{acc}%{i}."));
        heading = heading.add_level(
            Level::new(
                i - 1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new(level_text),
                LevelJc::new("left"),
            )
            .indent(None, None, None, None)
            .suffix(Space),
        )
    }
    let unordered = AbstractNumbering::new(UNORDERED_NUMBERING).add_level(
        Level::new(
            0,
            Start::new(1),
            NumberFormat::new("bullet"),
            LevelText::new("⚪"),
            LevelJc::new("left"),
        )
        .indent(None, None, None, None)
        .suffix(Space),
    );

    let mut docx = docx
        .add_abstract_numbering(heading)
        .add_abstract_numbering(unordered)
        .add_numbering(Numbering::new(HEADING_NUMBERING, HEADING_NUMBERING));
    let mut ordered_numbering = UNORDERED_NUMBERING;

    for (i, &ordered) in LIST_INFO.lock().as_deref().unwrap().iter().enumerate() {
        let id = UNORDERED_NUMBERING + i;
        docx = if ordered {
            ordered_numbering += 1;

            docx.add_abstract_numbering(
                AbstractNumbering::new(ordered_numbering).add_level(
                    Level::new(
                        0,
                        Start::new(1),
                        NumberFormat::new("decimal"),
                        LevelText::new("%1."),
                        LevelJc::new("left"),
                    )
                    .indent(None, None, None, None)
                    .suffix(Space),
                ),
            )
            .add_numbering(Numbering::new(id, ordered_numbering))
        } else {
            docx.add_numbering(Numbering::new(id, UNORDERED_NUMBERING))
        };
    }
    docx
}
