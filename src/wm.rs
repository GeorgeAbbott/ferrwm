use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::{get_keyboard_mapping, KeyButMask, Window, GetWindowAttributesReply};

use crate::config;
use crate::monitor::Monitor;
use crate::tag::Tag;
use crate::client::Client;
use crate::util::logf;
use crate::consts;

// Events
use x11rb::protocol::
    xproto::{
        ButtonPressEvent, ClientMessageEvent, ConfigureNotifyEvent, ConfigureRequestEvent,
        DestroyNotifyEvent, EnterNotifyEvent, ExposeEvent, FocusInEvent, KeyPressEvent,
        MapRequestEvent, MappingNotifyEvent, MotionNotifyEvent, PropertyNotifyEvent,
        UnmapNotifyEvent,
    };

// Represents the window manager itself. This holds 
// a Vec<Tag> which represent tags, as well as a 
// Vec<Client>. Each Tag then itself holds a 
// Vec<&'c Client> allow the window manager to 
// decide which clients to draw based on the selected
// tags.
pub struct WindowManager<'wm, 'rc> {
    conn: &'rc RustConnection,
    screen_num: usize,
    // tags: Vec<Tag<'wm>>, // This should be kept afaict, but ctbwwgt.
    running: bool,
    monitors: Vec<Monitor<'wm>>,
    current_mon: usize,
}

impl<'wm, 'rc> WindowManager<'wm, 'rc> {
    pub fn new(conn: &'rc RustConnection, screen_num: usize) -> Self {
        logf(format!("WindowManager::new -> screen_num: {}", screen_num).as_str());

        let mut tags = Vec::<Tag>::new();
        for tag in config::TAGS {
            tags.push(Tag::new(tag));
        }

        let mut monitors = Vec::<Monitor>::new();
        monitors.push(Monitor::new(conn, screen_num));
        let current_mon = 0;
        // TODO: populate with monitors

        Self {
            conn, 
            screen_num,
            //  tags, 
            // stext: "".to_string(),
            running: true,
            monitors,
            current_mon,
        }
    }

    /// TODO: implement. 
    pub fn manage(&mut self, window: Window, wa: GetWindowAttributesReply) {
        todo!()
    }

    /// Scans the X server for windows. If a window is found that is not 
    /// managed, i.e. not present as a client in ferrwm, then calls 
    /// WindowManager::manage to manage this client. 
    pub fn scan(&mut self) {
        todo!()
    }

    pub fn quit(&mut self) {
        logf("quit() called");
        self.running = false;
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::ButtonPress(e) => self.button_press(e),
            Event::ClientMessage(e) => self.client_message(e),
            Event::ConfigureRequest(e) => self.configure_request(e),
            Event::ConfigureNotify(e) => self.configure_notify(e),
            Event::DestroyNotify(e) => self.destroy_notify(e),
            Event::EnterNotify(e) => self.enter_notify(e),
            Event::Expose(e) => self.expose(e),
            Event::FocusIn(e) => self.focus_in(e),
            Event::KeyPress(e) => self.key_press(e),
            Event::MappingNotify(e) => self.mapping_notify(e),
            Event::MapRequest(e) => self.map_request(e),
            Event::MotionNotify(e) => self.motion_notify(e),
            Event::PropertyNotify(e) => self.property_notify(e),
            Event::UnmapNotify(e) => self.unmap_notify(e),
            _ => {}, // do nothing 
        };
    }

    pub fn run_event_loop(&mut self) {
        while let Ok(event) = self.conn.wait_for_event() {
            if !self.running { break; } 
            self.handle_event(event);
        }
    }

    /// Add a client to the currently selected monitor.
    pub fn add_client(&mut self, c: Client) {
        self.monitors[self.current_mon].add_client(c);
    }

    // TODO: Also needs root_id: this can be gotten from screen_num but perhaps
    // store for convenience.
    #[allow(dead_code)]
    /// Update the status text that appears in the top bar on the chosen 
    /// monitor; if the text is empty, then updates to "ferrwm: vernum".
    pub fn update_stext(&mut self, monitor: usize, text: impl Into<String>) { 
        let text = text.into();
        let mon = self.monitors[monitor];
        if text == "" {
            todo!(); 
        } else {
            todo!();
        }
    }

    /// Redraw bars on all monitors.
    pub fn draw_bars(&self) {
        for m in self.monitors {
            m.draw_bar();
        }
    }

    /// Spawn a window on the currently selected monitor.
    pub fn spawn_window(&mut self, window_name: impl Into<String>) {
        let window_name = window_name.into();
        
        logf(format!("called spawn_window with {}", 
                     window_name.as_str()).as_str());

        let c = Client::new(window_name);
        self.add_client(c);
    }

    /// Toggle whether or not the bar on the current monitor. is visible or not. 
    pub fn toggle_bar(&mut self) {
        logf("toggle_bar");
        self.monitors[self.current_mon].toggle_bar();
        // Is there anything else I need to do here? Refresh the screen?
    }

    #[allow(dead_code)]
    /// Toggle the bar on all monitors. This does not set all bars to on or off 
    /// but rather simply calls toggle_bar() on all monitors, inverting them. 
    pub fn toggle_all_bars(&mut self) {
        logf("toggle_all_bars");
        for m in self.monitors.iter_mut() {
            m.toggle_bar();
        }
    }

    #[allow(dead_code)]
    /// Assign the passed tag to the currently selected client on the currently 
    /// selected monitor.
    pub fn assign_tag(&mut self, tag: i32) {
        self.monitors[self.current_mon]
            .get_sel_client_mut()
            .assign_tag(tag)
    }

    /// TODO: do not reference Monitor by actual monitor but by number instead.
    pub fn move_client(from: Monitor, to: Monitor, client: Client) { todo!(); }
}

// impl block for all event handling functions
impl<'wm, 'rc> WindowManager<'wm, 'rc> {
    pub fn button_press(&self, event: ButtonPressEvent) {}
    pub fn client_message(&self, event: ClientMessageEvent) {}
    pub fn configure_request(&self, event: ConfigureRequestEvent) {}
    pub fn configure_notify(&self, event: ConfigureNotifyEvent) {}
    pub fn destroy_notify(&self, event: DestroyNotifyEvent) {}
    pub fn enter_notify(&self, event: EnterNotifyEvent) {}
    pub fn expose(&self, event: ExposeEvent) {}
    pub fn focus_in(&self, event: FocusInEvent) {}

    /// Handle a key press event.
    pub fn key_press(&mut self, event: KeyPressEvent) {
        logf("Entered key_press");
        
        let keypress: u8 = event.detail;
        let keystate = event.state; // for my reference, this is a mask of 
                                    // mods at the time. It's over at 
                                    // xcb.freedesktop.org/tutorial/events
                                    // some halfway down

        // FIXME: what is this let reply for? 
        // let reply = get_keyboard_mapping(self.conn, keypress, 1)
        //     .unwrap()
        //     .reply()
        //     .unwrap();

        self.act_on_keypress(KeyButMask::from(keystate), keypress);

        logf(format!("keypress value: {}", keypress).as_str());
    }
    pub fn mapping_notify(&self, event: MappingNotifyEvent) {}
    pub fn map_request(&self, event: MapRequestEvent) {}
    pub fn motion_notify(&self, event: MotionNotifyEvent) {}
    pub fn property_notify(&self, event: PropertyNotifyEvent) {}
    pub fn unmap_notify(&self, event: UnmapNotifyEvent) {}
}






