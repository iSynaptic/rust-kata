extern crate tempdir;
extern crate tantivy;
extern crate ansi_term;

use ansi_term::Colour::*;

mod indexing;

use indexing::InputDocument;
use indexing::DocumentIndex;

fn main() {
    program();
    ()
}
pub fn program() {
    print_intro();

    let input: Vec<InputDocument> = vec![InputDocument::new(
        "Never-ending Song".to_string(),
        "This is the song that never ends. It goes on and on my friend.".to_string()
    )];


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