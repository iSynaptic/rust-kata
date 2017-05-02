mod document_index;
mod document_loader;
mod input_document;

pub use self::document_index::DocumentIndex;
pub use self::document_loader::DocumentLoader;
pub use self::input_document::InputDocument;

pub fn search_by_index(term: &str, index: &DocumentIndex) -> Vec<String> {
    index.search(&term)
}