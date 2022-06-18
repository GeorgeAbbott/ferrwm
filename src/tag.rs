use crate::client::Client;

pub struct Tag<'c> {
    display: &'static str,
    clients: Vec<&'c Client>,
}

impl<'c> Tag<'c> {
    pub fn new(display: &'static str) -> Self {
        Self {
            display,
            clients: Vec::new(),

        }
    }
}
