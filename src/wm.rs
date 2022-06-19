use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::rust_connection::RustConnection;

use crate::config;
use crate::tag::Tag;
use crate::client::Client;

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
        }
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

    pub fn wait_event(&self) {
        while let Ok(event) = self.conn.wait_for_event() {
            self.handle_event(event);
        }
    }
}
