use crate::heading::max_heading_depth;
use docx_rs::{
    AbstractNumbering, Docx, IndentLevel, Level, LevelJc, LevelSuffixType, LevelText, NumberFormat,
    Numbering, NumberingId, Paragraph, Start,
};

const HEADING_NUMBERING: NumberingId = NumberingId { id: 1 };

/// 为标题添加多级列表编号，`depth` in 1..=6
pub fn heading_numbering(p: Paragraph, depth: u8) -> Paragraph {
    if let Some(depth) = depth.checked_sub(2) {
        p.numbering(HEADING_NUMBERING, IndentLevel::new(depth as _))
    } else {
        p
    }
}

pub fn add_numbering(docx: Docx) -> Docx {
    let mut abstruct = AbstractNumbering::new(HEADING_NUMBERING.id);
    for i in 1..max_heading_depth() {
        let level_text = (1..=i).fold(String::new(), |acc, i| format!("{acc}%{i}."));
        abstruct = abstruct.add_level(
            Level::new(
                i - 1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new(level_text),
                LevelJc::new("left"),
            )
            .indent(None, None, None, None)
            .suffix(LevelSuffixType::Space),
        )
    }

    docx.add_abstract_numbering(abstruct)
        .add_numbering(Numbering::new(HEADING_NUMBERING.id, HEADING_NUMBERING.id))
}
