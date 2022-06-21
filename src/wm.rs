use log::{trace, debug};

use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;
use x11rb::protocol::xproto::get_keyboard_mapping;

use crate::config;
use crate::tag::Tag;
use crate::client::Client;
use crate::utils::{logf, is_pressed, configkey_to_key};

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
    tags: Vec<Tag<'wm>>,
    clients: Vec<Client>,
    status_text: String, 
    running: bool,
}

impl<'wm, 'rc> WindowManager<'wm, 'rc> {
    pub fn new(conn: &'rc RustConnection, screen_num: usize) -> Self {
        let mut tags = Vec::new();

        for tag in config::TAGS {
            tags.push(Tag::new(tag));
        }

        Self {
            conn, 
            screen_num,
            tags, 
            clients: Vec::new(),
            status_text: "".to_string(),
            running: true,
        }
    }

    // I think this kind of implementation might cause issues with borrowck.
    pub fn quit(&mut self) {
        running = false;
    }

    fn handle_event(&self, event: Event) {
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

    pub fn run_event_loop(&self) {
        while let Ok(event) = self.conn.wait_for_event() {
            if !running { break; } 
            self.handle_event(event);
        }
    }

    // TODO: Also needs root_id: this can be gotten from screen_num but perhaps
    // store for convenience.
    pub fn update_status_text(&self, text: &str) { todo!(); }
    
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
    pub fn key_press(&self, event: KeyPressEvent) {
        trace!("Entered key_press");
        logf("Entered key_press");
        
        let keypress: u8 = event.detail;
        let keystate = event.state; // for my reference, this is a mask of 
                                // mods at the time. It's over at 
                                // xcb.freedesktop.org/tutorial/events
                                // some halfway down
        let reply = get_keyboard_mapping(self.conn, keypress, 1)
            .unwrap()
            .reply()
            .unwrap();
        // TODO: add this 

        // written like this so the warnings will stop hassling me, think this 
        // handle keypresses
        // Potential actions: 
        // spawn; togglebar; focusstack; incnmaster; setmfact; zoom; view;
        // killclient; setlayout; togglefloating; tag; focusmon; tagmon; quit.
        /*
         * focusstack: needs selmon, which presumably is managed by wm?
         * incnmaster: no state, but calls arrange(selmon) 
         * setmfact: again, uses selmon. 
         * zoom: again, selmon
         * view: again selmon
         * killclient: just uses selmon to check something selected else 
         * returns, and calls sendevent(selmon->sel, wmatom[WMDelete]) 
         * otherwise a bunch of X calls to presumably force kill it 
         *     sendevent: won't be hard to implement. Only X calls are 
         *     XGetWMProtocols, which cannot find an xcb equivalent for yet, and 
         *     XSendEvent -> xcb_send_event. 
         *
         * setlayout: only needs arg and selmon
         * togglefloating: only needs selmon
         * tag: only needs selmon. This is what assigned a window to a tag 
         * (or indeed, to multiple tags). This will definitely need to be in 
         * WindowManager class as it will need to add it to the Tag struct.
         * e.g. something like tag[3].assign(&client[n])
         * focusmon: access to mons, dirtomon and selmon (dirtomon: only 
         * mons and selmon)
         * tagmon: selmon and mons + sendmon + dirtomon
         * sendmon sends a client to a target monitor
         * quit: just sets running to zero
         * tf. implement these on WindowManager. 
         * How do we want multiple monitors to work? 
         */


        for e in crate::config::KEYBINDINGS {
            let mask = e.0;
            let key = configkey_to_key(&e.1);
            let action = e.2;

            if is_pressed((keypress, keystate), key, mask) {
                action(&e.3);
            }
        }
        debug!("Keypress value: {}", keypress);
        logf(format!("keypress value: {}", keypress).as_str());
    }
    pub fn mapping_notify(&self, event: MappingNotifyEvent) {}
    pub fn map_request(&self, event: MapRequestEvent) {}
    pub fn motion_notify(&self, event: MotionNotifyEvent) {}
    pub fn property_notify(&self, event: PropertyNotifyEvent) {}
    pub fn unmap_notify(&self, event: UnmapNotifyEvent) {}
}






