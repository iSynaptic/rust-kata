extern crate tantivy;
extern crate ansi_term;
extern crate regex;
extern crate time;
extern crate rand;

use std::env;
use std::io;
use std::io::Write;
use ansi_term::Colour::*;

use time::{Duration, PreciseTime};
use regex::Regex;
use rand::distributions::{IndependentSample, Range};

mod indexing;

use indexing::{InputDocument, DocumentIndex, DocumentLoader, SearchMethod };

struct SearchResults {
    duration: Duration,
    hits: Vec<(String, u64)>,
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

fn performance_test(docs: &Vec<InputDocument>, index: &DocumentIndex) {
    let filter_regex = Regex::new("^[\\w]+$").unwrap();
    let mut words : Vec<&str> = docs.iter()
        .take(30) // use words from the first 30 input documents
        .map(|x| x.contents())
        .flat_map(|x| x.split(" "))
        .filter(|x| filter_regex.is_match(x))
        .collect();
    
    // documentation indicates the vector must be sorted prior to dedupliccation
    words.sort_by(|&x, &y| x.cmp(y));
    words.dedup();

    print!("Executing naive performance benchmark.\n\nHow many iterations would you like to run: ");
    io::stdout().flush().unwrap();

    let mut iterations = String::new();

    io::stdin()
        .read_line(&mut iterations)
        .expect("Failed to read line");
    
    let iterations = iterations.trim().parse::<u32>();

    if iterations.is_err() {
        println!("You must enter a number. Exiting...\n");
        return;
    }

    let iterations = iterations.unwrap();

    println!("Results:\n");

    for method in SearchMethod::iter() {

        print!("  {}: ...", method);
        io::stdout().flush().unwrap();

        let search_fn = indexing::get_search_function(*method);
        
        // not going to warm up the code paths first
        let (_, duration) = time_work(||{
            let mut rng = rand::thread_rng();
            let between = Range::new(0usize, words.len() - 1);

            for _ in 1..iterations {
                let i = between.ind_sample(&mut rng);
                let search_word = words[i];

                search_fn(search_word, index, docs).unwrap();
            }
        });

        let duration = fmt_duration(&duration);
        println!("\u{0008}\u{0008}\u{0008}{}",duration);
    }

    println!();
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

        for hit in results.hits {
            println!("  {} - {} matches", hit.0, hit.1);
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
    println!("Search Method: 1) String Match 2) Regular Expression 3) Tantivy Index");
    print!("Enter method: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut method_answer)
        .expect("Failed to read line");

    println!();

    match method_answer.trim() {
        "1" => Ok(SearchMethod::StringMatch),
        "2" => Ok(SearchMethod::Regex),
        "3" => Ok(SearchMethod::TantivyIndex),
        _ => Err("Unrecognized method".to_string())
    }
}

fn time_search<F>(search_func: F) -> Result<SearchResults, String>
    where F: Fn() -> Result<Vec<(String, u64)>, String>
{
    let (results, duration) = 
        time_work(|| search_func());

    Ok(SearchResults {
           duration: duration,
           hits: results?,
       })
}

fn time_work<T, F>(work: F) -> (T, Duration) 
    where F: Fn() -> T {
    let start = PreciseTime::now();
    let results = work();
    let duration = start.to(PreciseTime::now());

    (results, duration)
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

fn fmt_duration(duration: &Duration) -> String {
    if duration.num_milliseconds() > 0 {
        format!("{} ms", duration.num_milliseconds())
    } else {
        format!("{} μs", duration.num_microseconds().unwrap())
    }
}