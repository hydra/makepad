use std::collections::HashMap;
use std::sync::Arc;
use crate::{
    makepad_widgets::*,
};
use crate::documents::DocumentKind;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::documents::text::*;
    import crate::documents::image::*;


    DocumentView = {{DocumentView}} {
        height: Fill,
        width: Fill,

        TextDocument = <TextDocumentView> {}
        ImageDocument = <ImageDocumentView> {}
    }
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct DocumentView {
    #[rust] document: Option<Arc<DocumentKind>>,
    #[rust] templates: HashMap<LiveId, LivePtr>,
    #[rust] child: Option<(LiveId, WidgetRef)>,
    #[walk] walk: Walk,
    #[rust] area: Area,
}

impl WidgetMatchEvent for DocumentView {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        println!("documentview. actions: {:?}", actions);
    }
}

impl Widget for DocumentView {

    fn handle_event(&mut self, cx:&mut Cx, event:&Event, scope:&mut Scope) {
        self.widget_match_event(cx, event, scope);

        // TODO do we even need to do this?
        if let Some((child_id, child)) = &self.child {
            scope.with_id(*child_id, |scope| {
                child.handle_event(cx, event, scope);
            })
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if let Some((_child_id, child)) = &self.child {
            child.draw_walk(cx, scope, walk)
        } else {
            DrawStep::done()
        }
    }
}

impl LiveHook for DocumentView {
    fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc {file_id} | ApplyFrom::UpdateFromDoc {file_id,..} => {
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    if nodes[index].value.is_enum() {
                        return index;
                    }
                    else {
                        let live_ptr = cx.live_registry.borrow().file_id_index_to_live_ptr(file_id, index);
                        self.templates.insert(id, live_ptr);
                    }
                }
                else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => ()
        }
        nodes.skip_node(index)
    }
}

impl DocumentView {
    pub fn set_document(&mut self, document_arc: Arc<DocumentKind>, cx: &mut Cx) {
        let template = match *document_arc {
            DocumentKind::TextDocument(_) => live_id!(TextDocument),
            DocumentKind::ImageDocument(_) => live_id!(ImageDocument),
        };


        self.document.replace(document_arc);

        let child_id = LiveId::unique();

        if let Some(ptr) = self.templates.get(&template) {
            let child_widget = WidgetRef::new_from_ptr(cx, Some(*ptr));
            self.child.replace((child_id, child_widget));
        }
        else {
            warning!("Template not found: {template}. Did you add it to the <DocumentView> instance in `live_design!{{}}`?");
        };
    }
}

impl WidgetNode for DocumentView {
    fn walk(&mut self, _cx:&mut Cx) -> Walk{
        self.walk
    }
    fn area(&self)->Area{self.area}

    fn redraw(&mut self, cx: &mut Cx){
        self.area.redraw(cx)
    }

    fn find_widgets(&self, _path: &[LiveId], _cached: WidgetCache, _results: &mut WidgetSet) {
        // TODO do we need this?
    }

    fn uid_to_widget(&self, uid:WidgetUid)->WidgetRef{
        if let Some((_child_id, child)) = &self.child {
            let child_widget = child.uid_to_widget(uid);
            if !child_widget.is_empty() {
                return child_widget
            }
        }
        WidgetRef::empty()
    }
}