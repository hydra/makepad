use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use makepad_widgets::*;
use crate::documents::DocumentKind;
use crate::documents::image::{ImageDocument, ImageDocumentView};

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

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    TextDocumentView = {{TextDocumentView}} {
        <RectView> {
            content = <Label> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TextDocumentView {
    #[deref] view: View,
    #[rust] document: Option<Arc<TextDocument>>,
}

impl Widget for TextDocumentView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {

        if let Some(document) = &self.document {
            self.label(id!(content)).set_text(document.content.as_str());
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl TextDocumentView {
    pub fn set_document(&mut self, document: Arc<TextDocument>) {
        self.document.replace(document);
    }
}
