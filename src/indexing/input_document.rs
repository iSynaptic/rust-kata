pub struct InputDocument {
    name: String,
    contents: String,
}

impl InputDocument {
    pub fn new(name: String, contents: String) -> InputDocument {
        InputDocument {
            name: name,
            contents: contents,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn contents(&self) -> &String {
        &self.contents
    }
}