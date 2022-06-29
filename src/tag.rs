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

// A tag bitmask.
pub struct Tags {
    tags: i32,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            tags: 0,
        }
    }

    const fn is_set(&self, tag: impl Into<i32>) -> Result<bool, ()> {
        if tag <= 0 || tag > 32 {
            Err(())
        } else {
            Ok(self.tags & (1 << tag))
        }
    }

    const fn set(&mut self, tag: impl Into<i32>) {
        self.tags &= tag;
    }

    const fn unset(&mut self, tag: impl Into<i32>) {
        self.tags &= !tag;
    }
}
