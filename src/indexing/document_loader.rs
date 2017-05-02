use std::path::Path;
use std::fs::File;
use std::io::Read;

use super::InputDocument;

pub struct DocumentLoader {}

impl DocumentLoader {
    pub fn load_from_directory(path: &Path) -> Result<Vec<InputDocument>, ::std::io::Error> {
        let read_dir = path.read_dir()?;
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

            let file_name = path.file_name().unwrap().to_str().unwrap();

            input_docs.push(InputDocument::new(file_name, &contents));
        }

        Ok(input_docs)
    }
}