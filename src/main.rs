extern crate ansi_term;
extern crate tantivy;
extern crate tempdir;

fn main() {
    rustkata::program();
    ()
}

mod rustkata {
    //use std::path::Path;
    use ansi_term::Colour::*;

    use tempdir::TempDir;
    use tantivy::Index;
    use tantivy::schema::*;
    use tantivy::query::QueryParser;
    use tantivy::collector::TopCollector;

    struct InputDocument {
        name: String,
        contents: String,
    }

    struct DocumentIndex {
        index: Index,
        query_parser: QueryParser,

        name_field: Field,
    }

    impl DocumentIndex {
        fn search(&self, q: &str) -> Vec<String> {
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

        fn build_index<T>(input_docs: T) -> Result<DocumentIndex, ::tantivy::Error>
            where T: Iterator<Item = InputDocument>
        {
            let index_dir = TempDir::new("rustkata")?;
            let index_path = index_dir.path();

            let schema = DocumentIndex::build_schema();

            let index = try!(Index::create(index_path, schema.clone()));

            let name_field = schema.get_field("name").unwrap();
            let contents_field = schema.get_field("contents").unwrap();

            let mut index_writer = try!(index.writer(50000000)); // 50MB buffer

            for input_doc in input_docs {
                let mut doc = Document::default();
                doc.add_text(name_field, &input_doc.name);
                doc.add_text(contents_field, &input_doc.contents);

                index_writer.add_document(doc);
            }

            try!(index_writer.commit());
            try!(index.load_searchers());

            let query_parser = QueryParser::new(index.schema(), vec![name_field]);

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

    pub fn program() {
        print_intro();

        let input: Vec<InputDocument> = vec![InputDocument {
                     name: "Never-ending Song".to_string(),
                     contents: "This is the song that never ends. It goes on and on my friend."
                         .to_string(),
                 }];


        if let Ok(index) = DocumentIndex::build_index(input.into_iter()) {
            let results = index.search("song");

            for result in results {
                println!("Result: {}", result);
            }
        }
        println!("Done!");
    }

    fn print_intro() {
        let intro = r#" (                         )                  
 )\ )              )    ( /(          )       
(()/(   (       ( /(    )\())   )  ( /(    )  
 /(_)) ))\  (   )\()) |((_)\ ( /(  )\())( /(  
(_))  /((_) )\ (_))/  |_ ((_))(_))(_))/ )(_)) 
| _ \(_))( ((_)| |_   | |/ /((_)_ | |_ ((_)_  
|   /| || |(_-<|  _|  | ' < / _` ||  _|/ _` | 
|_|_\ \_,_|/__/ \__|  |_|\_\\__,_| \__|\__,_|
"#;

        println!("{}", Red.bold().paint(intro));
    }
}