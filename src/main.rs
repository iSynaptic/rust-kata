extern crate tantivy;
extern crate ansi_term;
extern crate regex;
extern crate time;

use std::env;
use std::io;
use std::io::Write;
use ansi_term::Colour::*;

use regex::Regex;
use time::{Duration, PreciseTime};

mod indexing;

use indexing::InputDocument;
use indexing::DocumentIndex;
use indexing::DocumentLoader;

struct SearchResults {
    duration: Duration,
    documents: Vec<String>,
}

fn main() {
    print_intro();

    let sample_docs = get_sample_input_documents().expect("unable to load sample documents");

    println!("Building in-memory index of sample input files...\n");
    let index = DocumentIndex::build_index(sample_docs.iter()).expect("unable to build index");

    loop {
        let search_term = ask_for_search_term();
        if search_term.is_none() {
            break;
        }

        let search_term = search_term.unwrap();
        let results = prompt_for_method_and_search(&search_term, &index, &sample_docs);

        if results.is_err() {
            println!("{} - please try again.\n", results.err().unwrap());
            continue;
        }

        let results = results.unwrap();

        println!("Search results:\n");

        for result in results.documents {
            println!("  {}", result);
        }

        let duration = results.duration;

        if duration.num_milliseconds() > 0 {
            println!("\n Elapsed time: {} ms", duration.num_milliseconds());
        } else {
            println!("\n Elapsed time: {} μs",
                     duration.num_microseconds().unwrap());
        }


        if !ask_should_continue() {
            break;
        }
    }

    println!("Thank you, come again!");
}

fn ask_for_search_term() -> Option<String> {
    let mut search_term = String::new();
    print!("Enter the search term (press ENTER to exit): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut search_term)
        .expect("Failed to read line");

    println!();

    search_term = search_term.trim().to_string();

    if search_term == "" {
        return None;
    }

    Some(search_term)
}

fn prompt_for_method_and_search(term: &str,
                                index: &DocumentIndex,
                                docs: &Vec<InputDocument>)
                                -> Result<SearchResults, String> {

    let mut method_answer = String::new();
    println!("Search Method: 1) String Match 2) Regular Expression 3) Indexed");
    print!("Enter method: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut method_answer)
        .expect("Failed to read line");

    println!();

    let start = PreciseTime::now();

    match method_answer.trim() {
        "1" => {
            println!("{}{}",
                     Yellow.bold().paint("NOTE: "),
                     "search by 'String Match' is case sensitive.\n");

            let results = indexing::search_by_string_match(&term, docs);
            let duration = start.to(PreciseTime::now());

            Ok(SearchResults {
                   duration: duration,
                   documents: results,
               })
        }
        "2" => {
            let re = Regex::new(&term);
            if let Err(e) = re {
                let msg = format!("Expression invalid: {}", e);
                return Err(msg);
            };

            let re = re.ok().unwrap();

            let results = indexing::search_by_regex(&re, docs);
            let duration = start.to(PreciseTime::now());

            Ok(SearchResults {
                   duration: duration,
                   documents: results,
               })
        }
        "3" => {
            let results = indexing::search_by_index(&term, &index);
            let duration = start.to(PreciseTime::now());

            Ok(SearchResults {
                   duration: duration,
                   documents: results,
               })

        }
        _ => Err("Unrecognized search method".to_string()),
    }
}

fn ask_should_continue() -> bool {
    let mut continue_answer = String::new();

    println!("\nWould you like to search again?");
    print!("Enter 'no' to exit. Press ENTER to search again: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut continue_answer)
        .expect("Failed to read line");

    println!();

    if continue_answer.trim() == "no" {
        return false;
    }

    true
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