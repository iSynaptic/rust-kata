#[derive(Debug)]
pub struct InputDocument {
    name: String,
    contents: String,
}

impl InputDocument {
    pub fn new(name: &str, contents: &str) -> InputDocument {
        InputDocument {
            name: name.to_string(),
            contents: contents.to_string(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn contents(&self) -> &String {
        &self.contents
    }
}

#[cfg(test)]
mod tests {
    use super::InputDocument;

    #[test]
    fn input_document_created_correctly() {
        let doc = InputDocument::new("one", "sample contents");

        assert!(doc.name() == "one");
        assert!(doc.contents() == "sample contents");
    }
}