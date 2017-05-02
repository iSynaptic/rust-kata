use tantivy::Index;
use tantivy::schema::*;
use tantivy::query::QueryParser;
use tantivy::collector::TopCollector;

use InputDocument;

pub struct DocumentIndex {
    index: Index,
    query_parser: QueryParser,

    name_field: Field,
}

impl DocumentIndex {
    pub fn search(&self, q: &str) -> Vec<String> {
        let query = self.query_parser.parse_query(q).unwrap();
        let searcher = self.index.searcher();

        let mut top_collector = TopCollector::with_limit(10);
        searcher.search(&*query, &mut top_collector).unwrap();

        top_collector
            .docs()
            .into_iter()
            .flat_map(|x| searcher.doc(&x))
            .flat_map(|x| {
                          x.get_first(self.name_field)
                              .map(|f| f.text().to_string())
                      })
            .collect::<Vec<_>>()
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

        let query_parser = QueryParser::new(index.schema(), vec![contents_field]);

        Ok(DocumentIndex {
               index: index,
               query_parser: query_parser,
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
        assert_eq!(results[0], "one")
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
        assert_eq!(results[0], "one");
        assert_eq!(results[1], "three");
    }
}