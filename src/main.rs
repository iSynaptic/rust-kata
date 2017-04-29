extern crate ansi_term;
extern crate tantivy;
extern crate tempdir;

use std::path::Path;

use ansi_term::Colour::*;

use tempdir::TempDir;
use tantivy::Index;
use tantivy::schema::*;
use tantivy::query::QueryParser;
use tantivy::collector::TopCollector;

fn main() {
    print_intro();

    if let Ok(dir) = TempDir::new("temp_dir") {
        let path = dir.path();
        let schema = build_schema();

        if let Ok(index) = build_index(path, schema) {
            let searcher = index.searcher();
            let title_field = index.schema().get_field("title").unwrap();

            let query_parser = QueryParser::new(index.schema(), vec![title_field]);

            let query = query_parser.parse_query("song").unwrap();
            let mut top_collector = TopCollector::with_limit(10);

            searcher.search(&*query, &mut top_collector).unwrap();

            let results = top_collector.docs();


            for result in results {
                let doc = searcher.doc(&result).unwrap();
                match doc.get_first(title_field) {
                    Some(text) => println!("{}", text.text()),
                    _ => (),
                };
            }

            println!("Done!");
            ()
        }
    }
}


fn build_index(path: &Path, schema: Schema) -> Result<Index, tantivy::Error> {
    let index = try!(Index::create(path, schema.clone()));
    let title_field = schema.get_field("title").unwrap();
    let body_field = schema.get_field("body").unwrap();

    let mut index_writer = try!(index.writer(50_000_000)); // 50MB buffer

    let mut ne_song = Document::default();
    ne_song.add_text(title_field, "Never-ending Song");
    ne_song.add_text(body_field,
                     "This is the song that never ends. It goes on and on my friend.");


    index_writer.add_document(ne_song);

    try!(index_writer.commit());
    try!(index.load_searchers());

    Ok(index)
}

fn build_schema() -> Schema {
    let mut schema_builder = SchemaBuilder::default();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);

    schema_builder.build()
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
