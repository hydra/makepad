use makepad_widgets::*;


live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_widgets::vectorline::*;

    App = {{App}} {
        ui: <Window> {
            width: Fill, height: Fill

            caption_bar = {
                visible: true,
                caption_label = {
                    align = {x: 0}
                    label = {
                        text: "Makepad UI Tabbed Document Example"
                        margin: {left: (32)}
                    }
                }
            },

            body = <View> {
                width: Fill, height: Fill,
                flow: Down,

                toolbar = <View> {
                    flow: Right,
                    width: Fill, height: Fit,
                    home = <ButtonFlat> {
                        draw_icon: {
                            // FIXME just renders a grey box, not the SVG file.
                            svg_file: dep("crate://self/resources/home.svg"),
                        }
                        text: "Home"
                    }
                    new = <ButtonFlat> {
                        draw_icon: {
                            // FIXME just renders a grey box, not the SVG file.
                            svg_file: dep("crate://self/resources/folder-plus.svg"),
                        }
                        text: "New"
                    }
                    open = <ButtonFlat> {
                        draw_icon: {
                            // FIXME just renders a grey box, not the SVG file.
                            svg_file: dep("crate://self/resources/folder-open.svg"),
                        }
                        text: "Open"
                    }
                    close_all = <ButtonFlat> {
                        draw_icon: {
                            // FIXME just renders a grey box, not the SVG file.
                            svg_file: dep("crate://self/resources/square-x.svg"),
                        }
                        text: "Close All"
                    }
                }
                dock = <Dock> {
                    width: Fill,
                    height: Fill,
                    root = Tabs {
                        tabs: [tab_home]
                    }

                    tab_home = Tab {
                        name: "Home"
                        template: CloseableTab,
                        kind: HomeContainer
                    }

                    HomeContainer = <RectView> {
                        height: Fill, width: Fill,
                        align: {x: 0.5, y: 0.5}
                        flow: Down,
                        // FIXME doesn't render a unicode character, displays everything instead
                        <Label> {text: "\u{1F3E0}", padding: (10)}
                        <Label> {text: "Home", padding: (10)}
                    }
                }
            },
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, _cx: &mut Cx) {
        let _ui = self.ui.clone();
    }

    fn handle_action(&mut self, cx: &mut Cx, action: &Action) {
        println!("action: {:?}", action);
        let dock = self.ui.dock(id!(dock));

        if let Some(action) = action.as_widget_action() {
            match action.cast() {
                DockAction::TabCloseWasPressed(tab_id) => {
                    println!("closing tab: {:?}", tab_id);
                    dock.close_tab(cx, tab_id);
                    //dock.redraw(cx);
                },
                _ => {
                }
            }
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        //println!("actions: {:?}", actions);

        if self.ui.button(id!(home)).clicked(actions) {
            println!("adding home tab");

            let dock = self.ui.dock(id!(dock));
            // TODO what is this 'base' argument?
            let tab_id = dock.unique_tab_id(0);
            //let (tab_bar, pos) = dock.find_tab_bar_of_tab(live_id!(edit_first)).unwrap();
            let tab_bar = live_id!(root);
            dock.create_and_select_tab(cx, tab_bar, tab_id, live_id!(HomeContainer), "Home".to_string(), live_id!(CloseableTab), None);
        }

        let ui = self.ui.clone();

        for action in actions{
            self.handle_action(cx, action);
        }

    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {

}