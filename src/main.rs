extern crate tantivy;
extern crate ansi_term;
extern crate regex;
extern crate time;

use std::env;
use std::io;
use std::io::Write;
use ansi_term::Colour::*;

use time::{Duration, PreciseTime};

mod indexing;

use indexing::{InputDocument, DocumentIndex, DocumentLoader, SearchMethod };

struct SearchResults {
    duration: Duration,
    documents: Vec<String>,
}

fn main() {
    print_intro();

    let sample_docs = get_sample_input_documents().expect("unable to load sample documents");
    println!("Building in-memory index of sample input files...\n");

    let index = DocumentIndex::build_index(sample_docs.iter()).expect("unable to build index");

    let args: Vec<String> = env::args().collect();
    let mut mode: Option<&str> = None;

    if args.len() == 3 && args[1] == "--mode" {
        mode = Some(&args[2]);
    }

    match mode {
        Some("perf") => performance_test(&sample_docs, &index),
        _ => interactive_search(&sample_docs, &index),
    };

    println!("Thank you, come again!");
}

fn performance_test(_: &Vec<InputDocument>, _: &DocumentIndex) {
    println!("Do perf!!!!!");
}

fn interactive_search(docs: &Vec<InputDocument>, index: &DocumentIndex) {

    loop {
        let search_term = ask_for_search_term();
        if search_term.is_none() {
            break;
        }

        let search_term = search_term.unwrap();
        let results = prompt_for_method_and_search(&search_term, &index, docs);

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

    let search_method = prompt_for_method()?;
    let search_fn = indexing::get_search_function(search_method);

    if search_method == SearchMethod::StringMatch {
        println!("{}{}",
          Yellow.bold().paint("NOTE: "),
          "search by 'String Match' is case sensitive.\n");
    }

    time_search(|| search_fn(term, index, docs))
}


fn prompt_for_method() -> Result<SearchMethod, String> {
    let mut method_answer = String::new();
    println!("Search Method: 1) String Match 2) Regular Expression 3) Indexed");
    print!("Enter method: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut method_answer)
        .expect("Failed to read line");

    println!();

    match method_answer.trim() {
        "1" => Ok(SearchMethod::StringMatch),
        "2" => Ok(SearchMethod::Regex),
        "3" => Ok(SearchMethod::Index),
        _ => Err("Unrecognized method".to_string())
    }
}

fn time_search<F>(search_func: F) -> Result<SearchResults, String>
    where F: Fn() -> Result<Vec<String>, String>
{
    let start = PreciseTime::now();

    let results = search_func()?;
    let duration = start.to(PreciseTime::now());

    Ok(SearchResults {
           duration: duration,
           documents: results,
       })
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