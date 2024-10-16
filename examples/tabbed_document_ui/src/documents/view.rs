use std::sync::Arc;
use crate::{
    makepad_widgets::*,
};
use crate::documents::DocumentKind;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    DocumentView = {{DocumentView}} {
        <RectView> {
            height: Fill, width: Fill,

            text_document = <RectView> {
                visible: false,
                <Label> { text: "Text" }
            }
            image_document = <RectView> {
                visible: false,
                <Label> { text: "Image" }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct DocumentView {
    #[deref] view: View,
    #[rust] document: Option<Arc<DocumentKind>>
}

impl WidgetMatchEvent for DocumentView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        println!("documentview. actions: {:?}", actions);
    }
}

impl Widget for DocumentView {

    fn handle_event(&mut self, cx:&mut Cx, event:&Event, scope:&mut Scope) {
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {

        if let Some(document_arc) = self.document.as_ref() {
            match document_arc.as_ref() {
                DocumentKind::TextDocument(_) => {
                    self.view.view(id!(text_document)).set_visible(true);
                }
                DocumentKind::ImageDocument(_) => {
                    self.view.view(id!(image_document)).set_visible(true);

                }
            }
        }

        self.view.draw_walk(cx, scope, walk)
    }
}

impl DocumentView {
    pub fn set_document(&mut self, document_arc: Arc<DocumentKind>) {
        self.document.replace(document_arc);
    }
}