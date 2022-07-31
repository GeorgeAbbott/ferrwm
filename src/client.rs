use x11rb::protocol::xproto::Atom;

use crate::tag::Tags;
use crate::geom::Rect;

pub struct Client {
    name: String, 
    dimensions: Rect,
    old_dimensions: Rect,
    tags: Tags,
    // ... TODO: add
}

impl Client {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dimensions: Rect::new_zeroed(), // TODO: maybe actually set?
            old_dimensions: Rect::new_zeroed(),
            tags: Tags::new(),
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

    // TODO: document.
    pub fn set_state(&mut self, state: i64) { todo!(); }

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
