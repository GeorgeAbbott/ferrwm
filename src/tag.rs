use crate::client::Client;

pub struct Tag<'c> {
    display: &'static str,
    clients: Vec<&'c Client<'c>>,
}

impl<'c> Tag<'c> {
    pub fn new(display: &'static str) -> Self {
        Self {
            display,
            clients: Vec::new(),

        }
    }
}

/// A tag bitmask. Held by clients to state which tags they are attached to.
pub struct Tags {
    tags: i32,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            tags: 0,
        }
    }

    #[allow(dead_code)]
    const fn is_set(&self, tag: i32) -> Result<bool, ()> {
        if tag <= 0 || tag > 32 {
            Err(())
        } else {
            Ok((self.tags & (1 << tag)) != 0)
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, tag: i32) {
        self.tags &= tag;
    }

    #[allow(dead_code)]
    pub fn unset(&mut self, tag: i32) {
        self.tags &= !tag;
    }
}
