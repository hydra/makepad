use crate::{
    makepad_widgets::*,
    app::AppState,
};

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    HomeView = {{HomeView}} {
        <RectView> {
            height: Fill, width: Fill,
            align: {x: 0.5, y: 0.5}
            flow: Down,
            // FIXME doesn't render a unicode character, displays everything instead
            <Label> {text: "\u{1F3E0}", padding: (10)}
            <Label> {text: "Home", padding: (10)}
            show_on_startup_checkbox = <CheckBox> {text:"Show on startup"}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct HomeView {
    #[deref] view:View
}

impl WidgetMatchEvent for HomeView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        println!("homeview. actions: {:?}", actions);
        if let Some(check) = self.check_box(id!(show_on_startup_checkbox)).changed(actions) {

            let config = &mut scope.data.get_mut::<AppState>().unwrap().config;
            config.show_home_on_startup = check;

            println!("show_on_startup_checkbox changed");
        }
    }
}

impl Widget for HomeView {

    fn handle_event(&mut self, cx:&mut Cx, event:&Event, scope:&mut Scope){
        self.widget_match_event(cx, event, scope);
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let config = &mut scope.data.get_mut::<AppState>().unwrap().config;

        self.check_box(id!(show_on_startup_checkbox)).set_selected(cx, config.show_home_on_startup);

        while let Some(item) =  self.view.draw_walk(cx, scope, walk).step() {
            item.draw_all(cx, scope);
        }

        DrawStep::done()
    }
}