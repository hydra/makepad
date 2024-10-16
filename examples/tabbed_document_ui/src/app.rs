use makepad_widgets::*;
use makepad_widgets::desktop_button::DesktopButtonWidgetRefExt;
use crate::config;
use crate::config::Config;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_widgets::vectorline::*;
    import crate::home::*;

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
                        tabs: []
                    }

                    HomeContainer = <HomeView> {}
                }
            },
        }
    }
}

app_main!(App);

#[derive(Default)]
pub struct AppState {
    pub config: Config,
}


#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
    #[rust] state: AppState,
}

impl App {
    pub fn on_shutdown(&self) {
        println!("on_shutdown");

        config::save(&self.state.config);
    }

    pub fn add_home_tab(&self, cx: &mut Cx) {
        println!("adding home tab");

        let dock = self.ui.dock(id!(dock));
        // TODO what is this 'base' argument?
        let tab_id = dock.unique_tab_id(0);
        //let (tab_bar, pos) = dock.find_tab_bar_of_tab(live_id!(edit_first)).unwrap();
        let tab_bar = live_id!(root);
        dock.create_and_select_tab(cx, tab_bar, tab_id, live_id!(HomeContainer), "Home".to_string(), live_id!(CloseableTab), None);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        let _ui = self.ui.clone();

        self.state.config = config::load();

        if self.state.config.show_home_on_startup {
            self.add_home_tab(cx);
        }
    }

    fn handle_shutdown(&mut self, _cx: &mut Cx) {
        // FIXME this function is never called, on Windows 11 the Event::Shutdown is never generated. See windows.rs.
        self.on_shutdown();
    }

    fn handle_action(&mut self, cx: &mut Cx, action: &Action) {
        println!("action: {:?}", action);
        let dock = self.ui.dock(id!(dock));

        // if let Some(action) = action.as_widget_action() {
        //     match action.cast() {}
        // }

        if let Some(action) = action.as_widget_action() {
            match action.cast() {
                DockAction::TabCloseWasPressed(tab_id) => {
                    println!("closing tab: {:?}", tab_id);
                    dock.close_tab(cx, tab_id);
                },
                _ => {
                }
            }
        }
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        println!("actions: {:?}", actions);

        // HACK for `handle_shutdown` never being called on windows
        if self.ui.desktop_button(id!(caption_bar.windows_buttons.close)).clicked(actions) {
            println!("close clicked");
            self.on_shutdown();
        }

        if self.ui.button(id!(home)).clicked(actions) {
            self.add_home_tab(cx);
        }

        let ui = self.ui.clone();

        for action in actions{
            self.handle_action(cx, action);
        }

    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::home::live_design(cx);
    }
}
impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::with_data(&mut self.state));
    }
}

impl App {

}