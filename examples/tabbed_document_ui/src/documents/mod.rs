use std::fmt::{Debug, Formatter};
use slotmap::new_key_type;
use crate::documents::image::ImageDocument;
use crate::documents::text::TextDocument;

pub mod text;
pub mod image;
pub mod view;

pub enum DocumentKind {
    TextDocument(TextDocument),
    ImageDocument(ImageDocument),
}

impl Debug for DocumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TextDocument(_) => f.write_str("TextDocument"),
            Self::ImageDocument(_) => f.write_str("ImageDocument"),
        }
    }
}

new_key_type! {
    pub struct DocumentKey;
}
