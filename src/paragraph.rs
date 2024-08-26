use crate::{
    docx, md, numbering::heading_numbering, style::heading_style, text::to_paragraph_children,
};
use std::sync::atomic::{AtomicU8, Ordering::Relaxed};

static MAX_HEADING_DEPTH: AtomicU8 = AtomicU8::new(0);

pub fn max_heading_depth() -> usize {
    MAX_HEADING_DEPTH.load(Relaxed) as _
}

pub fn from_heading(heading: md::Heading) -> docx::Paragraph {
    let md::Heading {
        depth, children, ..
    } = heading;
    MAX_HEADING_DEPTH.fetch_max(depth, Relaxed);

    let mut p = docx::Paragraph::new();
    p.children.extend(to_paragraph_children(children));
    p = heading_style(p, depth);
    p = heading_numbering(p, depth);
    p
}

pub fn from_paragraph(md::Paragraph { children, .. }: md::Paragraph) -> docx::Paragraph {
    let mut p = docx::Paragraph::new();
    p.children.extend(to_paragraph_children(children));
    p
}

pub fn from_link(md::Link { children, url, .. }: md::Link) -> docx::Hyperlink {
    let mut h = docx::Hyperlink::new(url, docx::HyperlinkType::External);
    h.children.extend(to_paragraph_children(children));
    h
}
