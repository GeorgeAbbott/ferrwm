use x11rb::{
    connection::Connection,
    protocol::xproto::{create_window, map_window, CreateWindowAux, Screen, Visualid, WindowClass, CreateGCAux, Drawable, create_gc, ConnectionExt, Rectangle, create_colormap, ColormapAlloc, alloc_color, AllocColorReply},
    rust_connection::RustConnection,
    COPY_DEPTH_FROM_PARENT,
};

use crate::monitor::Monitor;

// Represent the bar. ATM a really bad API but I want to test it a little.
pub struct Bar<'rc> {
    conn: &'rc RustConnection,
    root: Drawable,
    stext: String,
    id_bar: u32,
    id_gc: u32,
    cmap: u32,
    id_parent: u32,
    width: u16,
    height: u16,
    border_width: u16,
    visual: Visualid,
}

impl<'rc> Bar<'rc> {
    // TODO: don't pass screen, pass individual members of screen instead.
    pub fn new(conn: &'rc RustConnection, screen: &Screen) -> Self {
        let root = screen.root;
        let cmap   = screen.default_colormap;
        let visual = screen.root_visual;
        
        let id_bar = conn.generate_id().expect("Bar::new - generate_id failed");
        let id_gc = conn.generate_id().expect("Bar::new - gc generate_id failed");
        let red = conn.alloc_color(cmap, u16::MAX, u16::MIN, u16::MIN)
            .unwrap()
            .reply()
            .unwrap()
            .pixel;

        create_gc(conn, id_gc, root, &CreateGCAux::new()
                  .background(red));

        Self {
            conn,
            root,
            stext: "".to_string(),
            id_bar,
            id_gc,
            cmap,
            id_parent: screen.root,
            width: screen.width_in_pixels,
            height: 50,
            border_width: 5,
            visual: screen.root_visual,
        }
    }

    /// Draw the bar. 
    pub fn draw(&self) {
        self.conn 
            .poly_rectangle(self.root, self.id_gc, 
                            &[Rectangle{ x: 0, y: 0, width: self.width, height: self.height }]);
    }

    /* As far as I am concerned this is old code and can be deleted; TODO check */
    // pub fn create(&self) {
    //     create_window(
    //         self.conn,
    //         COPY_DEPTH_FROM_PARENT,
    //         self.id_bar,
    //         self.id_parent,
    //         0,
    //         0,
    //         self.width,
    //         self.height,
    //         self.border_width,
    //         WindowClass::INPUT_OUTPUT,
    //         self.visual,
    //         &CreateWindowAux::new(),
    //     )
    //     .expect("Bar::create - create_window failed");
    // }
}
