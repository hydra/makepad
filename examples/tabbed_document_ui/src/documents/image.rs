use std::path::PathBuf;

pub struct ImageDocument {
    path: PathBuf,
    coordinate: Option<(usize, usize)>
}

impl ImageDocument {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            coordinate: None,
        }
    }
}
