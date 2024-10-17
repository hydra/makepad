use std::fs;
use std::path::PathBuf;
use makepad_widgets::*;

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
            <Label> { text: "Text" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TextDocumentView {
    #[deref] view: View
}

impl Widget for TextDocumentView {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}