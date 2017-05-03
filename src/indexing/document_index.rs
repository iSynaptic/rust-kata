use tantivy::{DocAddress, Searcher, Index, Term, SegmentPostingsOption};
use tantivy::schema::*;
use tantivy::postings::{DocSet, Postings, SkipResult};
use tantivy::query::TermQuery;
use tantivy::collector::TopCollector;

use InputDocument;

pub struct DocumentIndex {
    index: Index,
    name_field: Field,
}

impl DocumentIndex {
    pub fn search(&self, term: &str) -> Vec<(String, u64)> {
        let contents_field = self.index.schema().get_field("contents").unwrap();

        let term = Term::from_field_text(contents_field, term);
        let query = TermQuery::new(term.clone(), SegmentPostingsOption::Freq);
        
        let searcher = self.index.searcher();

        let mut top_collector = TopCollector::with_limit(10);
        searcher.search(&query, &mut top_collector).unwrap();

        // This is a bit messy. It would probably benefit from creating some intermediate
        // types.  This bit of code exists primarily to satisfy the requirement that results be
        // based on term frequency (ie. number of occurences), and not a TF-IDF based score.
        // Technically, the search is still based on the TF-IDF score, but the results are 
        // strictly term frequency.
        top_collector
            .docs()
            .into_iter()
            .map(|x| (x, DocumentIndex::extract_termfreq(&searcher, &term, &x.clone()).unwrap())) // get term frequency
            .map(|x| (searcher.doc(&x.0).unwrap(), x.1)) // retreive the document 
            .map(|x| {( // extract the name from the document
                x.0.get_first(self.name_field).map(|f| f.text().to_string()).unwrap(),
                x.1
            )})
            .collect()
    }

    fn extract_termfreq(searcher: &Searcher, term: &Term, doc_address: &DocAddress) -> Option<u64> {
        searcher
            .segment_reader(doc_address.segment_ord() as usize)
            .read_postings(term, SegmentPostingsOption::Freq)
            .and_then(|mut postings| {
                if postings.skip_next(doc_address.doc()) == SkipResult::Reached {
                    Some(postings.term_freq() as u64)
                }
                else {
                    None
                }
            })
    }

    pub fn build_index<'a, T>(input_docs: T) -> Result<DocumentIndex, ::tantivy::Error>
        where T: Iterator<Item = &'a InputDocument>
    {
        let schema = DocumentIndex::build_schema();

        let index = Index::create_in_ram(schema.clone());

        let name_field = schema.get_field("name").unwrap();
        let contents_field = schema.get_field("contents").unwrap();

        let mut index_writer = try!(index.writer(50000000)); // 50MB buffer

        for input_doc in input_docs {
            let mut doc = Document::default();
            doc.add_text(name_field, input_doc.name());
            doc.add_text(contents_field, input_doc.contents());

            index_writer.add_document(doc);
        }

        try!(index_writer.commit());
        try!(index.load_searchers());

        Ok(DocumentIndex {
               index: index,
               name_field: name_field,
           })
    }

    fn build_schema() -> Schema {
        let mut schema_builder = SchemaBuilder::default();
        schema_builder.add_text_field("name", TEXT | STORED);
        schema_builder.add_text_field("contents", TEXT);

        schema_builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::DocumentIndex;
    use super::InputDocument;

    #[test]
    fn can_build_empty_index() {
        let docs: Vec<InputDocument> = vec![];

        let index = DocumentIndex::build_index(docs.iter());
        assert!(index.is_ok());
    }

    #[test]
    fn can_index_one_document() {
        let docs = vec![InputDocument::new("one", "sample content")];

        let index = DocumentIndex::build_index(docs.iter());
        assert!(index.is_ok());
    }

    #[test]
    fn can_search_index_with_one_document() {
        let docs = vec![InputDocument::new("one", "sample content")];

        let index = DocumentIndex::build_index(docs.iter()).unwrap();
        let results = index.search("sample");

        assert!(results.len() == 1);
        assert_eq!(results[0].0, "one");
        assert_eq!(results[0].1, 1)
    }

    #[test]
    fn can_search_index_with_many_document() {
        let docs = vec![InputDocument::new("one",
                                           "Consequently, spacecraft at warp velocity can continue to interact with objects in \"normal space\"."),
                        InputDocument::new("two",
                                           "The Hitchhiker's Guide to the Galaxy is a comedy science fiction series created by Douglas Adams."),
                        InputDocument::new("three",
                                           "The Infinite Improbability Drive is a faster-than-light drive. In the 2005 film, for instance, the first time the Improbability Drive is used, the entire ship, after traveling at extreme velocity, arrives at its destination, ends up as a giant ball of yarn for a few seconds, and the main characters are rendered as animated yarn dolls.")];

        let index = DocumentIndex::build_index(docs.iter()).unwrap();
        let results = index.search("velocity");

        assert!(results.len() == 2);
        assert_eq!(results[0].0, "one");
        assert_eq!(results[0].1, 1);

        assert_eq!(results[1].0, "three");
        assert_eq!(results[1].1, 1);
    }
}