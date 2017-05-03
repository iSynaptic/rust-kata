mod document_index;
mod document_loader;
mod input_document;

pub use self::document_index::DocumentIndex;
pub use self::document_loader::DocumentLoader;
pub use self::input_document::InputDocument;

use regex::Regex;


#[derive(Copy, Clone, PartialEq)]
pub enum SearchMethod {
    StringMatch,
    Regex,
    Index
}

pub type SearchFn = Fn(&str, &DocumentIndex, &Vec<InputDocument>) ->
    Result<Vec<String>, String>;


pub fn get_search_function(method: SearchMethod) -> Box<SearchFn> {
   match method {
        SearchMethod::StringMatch => Box::new(move |t,_,d| Ok(search_by_string_match(t, d))),
        SearchMethod::Regex => Box::new(move |t,_,d| search_by_regex(t, d)),
        SearchMethod::Index => Box::new(move |t,i,_| Ok(search_by_index(t, i)))
    }
}

fn search_by_index(term: &str, index: &DocumentIndex) -> Vec<String> {
    index.search(&term)
}

fn search_by_string_match(term: &str, docs: &Vec<InputDocument>) -> Vec<String> {
    let mut matches = docs.iter()
        .map(|doc| (doc, doc.contents().matches(term).count()))
        .filter(|x| x.1 > usize::min_value())
        .collect::<Vec<(&InputDocument, usize)>>();

    matches.sort_by(|&x, &y| x.1.cmp(&y.1));
    matches.iter().map(|x| x.0.name().to_string()).collect()
}

fn search_by_regex(term: &str, docs: &Vec<InputDocument>) -> Result<Vec<String>, String> {
    let re = Regex::new(term);
    if let Err(e) = re {
        let msg = format!("Expression invalid: {}", e);
        return Err(msg);
    };

    let re = re.ok().unwrap();

    let mut matches = docs.iter()
        .map(|doc| (doc, re.find_iter(doc.contents()).count()))
        .filter(|x| x.1 > usize::min_value())
        .collect::<Vec<(&InputDocument, usize)>>();

    matches.sort_by(|&x, &y| x.1.cmp(&y.1));
    Ok(matches.iter().map(|x| x.0.name().to_string()).collect())
}