use std::fs;
use std::path::PathBuf;

pub struct TextDocument {
    path: PathBuf,
    content: String,
}

impl TextDocument {
    pub fn new(path: PathBuf) -> Self {

        let content = fs::read_to_string(&path).unwrap();

        Self {
            path,
            content,
        }
    }
}
