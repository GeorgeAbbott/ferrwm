use crate::config;
use crate::tag::Tag;
use crate::client::Client;

// Represents the window manager itself. This holds 
// a Vec<Tag> which represent tags, as well as a 
// Vec<Client>. Each Tag then itself holds a 
// Vec<&'c Client> allow the window manager to 
// decide which clients to draw based on the selected
// tags.
pub struct WindowManager<'wm> {
    tags: Vec<Tag<'wm>>,
    clients: Vec<Client>,
}

impl<'wm> WindowManager<'wm> {
    pub fn new() -> Self {
        let mut tags = Vec::new();

        for tag in config::TAGS {
            tags.push(Tag::new(tag));
        }

        Self {
            tags, 
            clients: Vec::new(),
        }
    }
}
