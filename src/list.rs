use docx_rs::Paragraph;
use markdown::mdast::List;

pub fn from_list(list: List) -> Vec<Paragraph> {
    let List {
        children,
        ordered,
        start,
        ..
    } = list;
    todo!()
}
