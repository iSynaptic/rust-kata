extern crate tantivy;
extern crate ansi_term;

use std::env;
use ansi_term::Colour::*;

mod indexing;

use indexing::InputDocument;
use indexing::DocumentIndex;
use indexing::DocumentLoader;

fn main() {
    print_intro();
    let index = build_index_from_sample_input().expect("unable to build index from sample input");

    let results = index.search("warp");

    for result in results {
        println!("Result: {}", result);
    }
    println!("Done!");
}

fn build_index_from_sample_input() -> Result<DocumentIndex, std::io::Error> {
    println!("Building in-memory index of sample input files...");

    let mut dir = env::current_dir().unwrap();
    dir.push("sample_input");

    let input_docs = DocumentLoader::load_from_directory(dir.as_path())?;
    let result = DocumentIndex::build_index(input_docs.into_iter());

    Ok(result.expect("unable to build index"))
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