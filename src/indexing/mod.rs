mod document_index;
mod document_loader;
mod input_document;

pub use self::document_index::DocumentIndex;
pub use self::document_loader::DocumentLoader;
pub use self::input_document::InputDocument;

pub fn search_by_index(term: &str, index: &DocumentIndex) -> Vec<String> {
    index.search(&term)
}

pub fn search_by_string_match(term: &str, docs: &Vec<InputDocument>) -> Vec<String> {
    let mut matches = docs.iter()
        .map(|doc| (doc, doc.contents().matches(term).count()))
        .filter(|x| x.1 > usize::min_value())
        .collect::<Vec<(&InputDocument, usize)>>();

    matches.sort_by(|&x, &y| x.1.cmp(&y.1));
    matches.iter().map(|x| x.0.name().to_string()).collect()
}

pub fn search_by_regex(regex: &str, docs: &Vec<InputDocument>) -> Vec<String> {
    vec![]
}