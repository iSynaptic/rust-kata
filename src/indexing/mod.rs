mod document_index;
mod document_loader;
mod input_document;

pub use self::document_index::DocumentIndex;
pub use self::document_loader::DocumentLoader;
pub use self::input_document::InputDocument;

use std::fmt;
use regex::Regex;
use std::slice::Iter;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum SearchMethod {
    StringMatch,
    Regex,
    Index
}

impl SearchMethod {
    pub fn iter() -> Iter<'static, SearchMethod> {
        static METHODS: [SearchMethod;  3] = [
            SearchMethod::StringMatch,
            SearchMethod::Regex,
            SearchMethod::Index,
        ];

        METHODS.into_iter()
    }   
}

impl fmt::Display for SearchMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match *self {
            SearchMethod::StringMatch => "String Match",
            SearchMethod::Regex => "Regular Expression",
            SearchMethod::Index => "Indexed",
        };
        write!(f, "{}", text)
    }
}

pub type SearchFn = Fn(&str, &DocumentIndex, &Vec<InputDocument>) ->
    Result<Vec<(String, u64)>, String>;


pub fn get_search_function(method: SearchMethod) -> Box<SearchFn> {
   let underlying : Box<SearchFn> = match method {
        SearchMethod::StringMatch => Box::new(move |t,_,d| Ok(search_by_string_match(t, d))),
        SearchMethod::Regex => Box::new(move |t,_,d| search_by_regex(t, d)),
        SearchMethod::Index => Box::new(move |t,i,_| Ok(search_by_index(t, i)))
    };

    Box::new(move |t,i,d|{
        let results = underlying(t,i,d);
        if results.is_err() {
            return results;
        }

        let mut results = results.unwrap();
        results.sort_by(|ref x, ref y| y.1.cmp(&x.1));

        Ok(results)
    })
}

fn search_by_index(term: &str, index: &DocumentIndex) -> Vec<(String, u64)> {
    index.search(&term)
        .into_iter()
        .map(|x| (x, 0))
        .collect::<Vec<(String, u64)>>()
}

fn search_by_string_match(term: &str, docs: &Vec<InputDocument>) -> Vec<(String, u64)> {
    let matches = docs.iter()
        .map(|doc| (doc, doc.contents().matches(term).count()))
        .filter(|x| x.1 > usize::min_value())
        .collect::<Vec<(&InputDocument, usize)>>();

    matches
        .into_iter()
        .map(|x| (x.0.name().to_string(), x.1 as u64))
        .collect()
}

fn search_by_regex(term: &str, docs: &Vec<InputDocument>) -> Result<Vec<(String, u64)>, String> {
    let re = Regex::new(term);
    if let Err(e) = re {
        let msg = format!("Expression invalid: {}", e);
        return Err(msg);
    };

    let re = re.ok().unwrap();

    let matches = docs.iter()
        .map(|doc| (doc, re.find_iter(doc.contents()).count()))
        .filter(|x| x.1 > usize::min_value())
        .collect::<Vec<(&InputDocument, usize)>>();

    Ok(matches
        .into_iter()
        .map(|x| (x.0.name().to_string(), x.1 as u64))
        .collect())
}