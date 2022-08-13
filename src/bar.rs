use x11rb::{
    connection::Connection,
    protocol::xproto::{create_window, map_window, CreateWindowAux, Screen, Visualid, WindowClass},
    rust_connection::RustConnection,
    COPY_DEPTH_FROM_PARENT,
};

use crate::monitor::Monitor;

// Represent the bar. ATM a really bad API but I want to test it a little.
pub struct Bar<'rc> {
    conn: &'rc RustConnection,
    stext: String,
    id_bar: u32,
    id_parent: u32,
    width: u16,
    height: u16,
    border_width: u16,
    visual: Visualid,
}

impl<'rc> Bar<'rc> {
    // TODO: don't pass screen, pass individual members of screen instead.
    pub fn new(conn: &'rc RustConnection, screen: &Screen) -> Self {
        let id_bar = conn.generate_id().expect("Bar::new - generate_id failed");
        Self {
            conn,
            stext: "".to_string(),
            id_bar,
            id_parent: screen.root,
            width: screen.width_in_pixels,
            height: 50,
            border_width: 5,
            visual: screen.root_visual,
        }
    }

    pub fn create(&self) {
        create_window(
            self.conn,
            COPY_DEPTH_FROM_PARENT,
            self.id_bar,
            self.id_parent,
            0,
            0,
            self.width,
            self.height,
            self.border_width,
            WindowClass::INPUT_OUTPUT,
            self.visual,
            &CreateWindowAux::new(),
        )
        .expect("Bar::create - create_window failed");
    }

    pub fn map(&self) {
        map_window(self.conn, self.id_bar).expect("Bar::map - failed");
    }
}
