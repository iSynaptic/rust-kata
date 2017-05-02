extern crate tantivy;
extern crate ansi_term;

use std::env;
use std::io;
use std::io::Write;
use ansi_term::Colour::*;

mod indexing;

use indexing::InputDocument;
use indexing::DocumentIndex;
use indexing::DocumentLoader;

fn main() {
    print_intro();

    let sample_docs = get_sample_input_documents().expect("unable to load sample documents");

    println!("Building in-memory index of sample input files...\n");
    let index = DocumentIndex::build_index(sample_docs.iter()).expect("unable to build index");


    loop {
        print!("Enter the search term (press ENTER to exit): ");
        io::stdout().flush().unwrap();

        let mut search_term = String::new();
        io::stdin()
            .read_line(&mut search_term)
            .expect("Failed to read line");

        println!();

        search_term = search_term.trim().to_string();

        if search_term == "" {
            break;
        }

        //let search_term = search_term;
        let results = index.search(&search_term);

        println!("Search results:\n");

        for result in results {
            println!("  {}", result);
        }

        println!("\nWould you like to search again?");
        print!("Type 'no' to exit. Press ENTER to search again: ");
        io::stdout().flush().unwrap();

        let mut continue_answer = String::new();
        io::stdin()
            .read_line(&mut continue_answer)
            .expect("Failed to read line");

        println!();

        if continue_answer.trim() == "no" {
            break;
        }

    }

    println!("Thank you, come again!");
}

fn get_sample_input_documents() -> Result<Vec<InputDocument>, std::io::Error> {
    let mut dir = env::current_dir().unwrap();
    dir.push("sample_input");

    DocumentLoader::load_from_directory(dir.as_path())
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