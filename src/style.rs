#[inline]
pub fn heading_style_id(depth: u8) -> String {
    format!("Heading{depth}")
}

pub const INLINE_CODE_STYLE_ID: &str = "InlineCode";
