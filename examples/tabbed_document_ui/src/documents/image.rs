use std::path::PathBuf;
use std::sync::Arc;
use makepad_widgets::*;

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

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ImageDocumentView = {{ImageDocumentView}} {
        <RectView> {
                <Label> { text: "Image" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageDocumentView {
    #[deref] view: View,
    #[rust] document: Option<Arc<ImageDocument>>,
}

impl Widget for ImageDocumentView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ImageDocumentView {
    pub fn set_document(&mut self, document: Arc<ImageDocument>) {
        self.document.replace(document);
    }
}
