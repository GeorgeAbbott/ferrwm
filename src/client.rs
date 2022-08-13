use x11rb::protocol::xproto::{Atom, change_property, PropMode, Window};
use x11rb::rust_connection::RustConnection;

use crate::tag::Tags;
use crate::geom::Rect;

pub struct Client<'rc> {
    conn: &'rc RustConnection,
    name: String, 
    dimensions: Rect,
    old_dimensions: Rect,
    tags: Tags,
    urgent: bool,
    window: Window, // TODO: add 
    // ... TODO: add
}

impl<'rc> Client<'rc> {
    pub fn new(conn: &RustConnection, name: String) -> Self {
        Self {
            conn,
            name,
            dimensions: Rect::new_zeroed(), // TODO: maybe actually set?
            old_dimensions: Rect::new_zeroed(),
            tags: Tags::new(),
            urgent: false,
        }
    }

    /// Assign the tag for this client.
    pub fn assign_tag(&mut self, tag: i32) {
        self.tags.set(tag);
    }

    /// Unassign the tag for this client.
    pub fn unassign_tag(&mut self, tag: i32) {
        self.tags.unset(tag);
    }

    #[allow(dead_code)]
    /// Set the focus for a client.
    pub fn set_focus(&mut self) {}

    #[allow(dead_code)]
    /// Send an event to the client, and returns a bool for 
    /// whether the event exists. 
    // TODO: check the above description is accurate.
    pub fn send_event(&self, proto: Atom) -> bool { todo!(); }

    /// Set state. ATM a carbon-copy of C implementation, TODO verify. 
    pub fn set_state(&mut self, state: i64) { 
            change_property(self.conn, PropMode::REPLACE, self.window, 
                            1, // [[wmatom[WMState]]], // what is in C, TODO: make into Rust
                            1, // [[wmatom[WMState]]], 
                            32, // Not quite sure why, seems a bit magic
                            2, // length of data
                            &[state, x11rb::NONE]);
    }

    /// Set whether window urgent or not. 
    pub fn set_urgent(&mut self, urgent: bool) { 
        todo!(); 
    }

    /// Returns size hints, and whether any of these hints 
    /// are different to the client's size.
    pub fn apply_size_hints(&self, interact: bool) -> (Rect, bool) {
        todo!();
    }

    /// Calls apply_size_hints, then resize.
    pub fn hinted_resize(&mut self, rect: Rect, interact: bool) {

    }

    /// Resize the client to the given size.
    pub fn resize(&mut self, rect: Rect) { todo!(); }




}
