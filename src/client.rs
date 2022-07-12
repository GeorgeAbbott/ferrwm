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
}
