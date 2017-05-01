extern crate tantivy;
extern crate ansi_term;

use std::env;
use std::fs::File;
use std::io::Read;
use ansi_term::Colour::*;

mod indexing;

use indexing::InputDocument;
use indexing::DocumentIndex;

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

    let read_dir = dir.read_dir()?;
    let mut input_docs: Vec<InputDocument> = vec![];

    for entry in read_dir {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let mut f = File::open(&path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let contents = contents;

        let file_name = path.to_str().unwrap().to_string();

        input_docs.push(InputDocument::new(file_name, contents));
    }

    let result = DocumentIndex::build_index(input_docs.into_iter());
    Ok(result.expect("bad"))
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