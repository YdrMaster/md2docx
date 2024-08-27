use super::{
    docx, md,
    numbering::heading_numbering,
    style::{body_text_style, caption_style, heading_style, image_style},
    text::to_paragraph_children,
    Ast,
};
use std::{
    path::Path,
    sync::atomic::{AtomicU8, Ordering::Relaxed},
};

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
    p = heading_numbering(p, depth);
    heading_style(p, depth)
}

pub fn from_paragraph(p: md::Paragraph, dir: &Path) -> (docx::Paragraph, Option<docx::Paragraph>) {
    let md::Paragraph { children, .. } = p;

    let mut p = docx::Paragraph::new();
    if let [Ast::Image(md::Image { alt, url, .. })] = &*children {
        let pic = std::fs::read(url).or(std::fs::read(dir.join(url))).unwrap();
        (
            image_style(p.add_run(docx::Run::new().add_image(docx::Pic::new(&pic)))),
            Some(caption_style(
                docx::Paragraph::new().add_run(docx::Run::new().add_text(alt)),
            )),
        )
    } else {
        p.children.extend(to_paragraph_children(children));
        (body_text_style(p), None)
    }
}
