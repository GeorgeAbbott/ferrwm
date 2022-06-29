use x11rb::{rust_connection::RustConnection, protocol::xproto::{create_pixmap, create_gc,
CreateGCAux, free_pixmap, free_gc}, connection::Connection};



pub struct Draw<'a> {
    conn: &'a RustConnection,
    screen: usize,
    root: u32,
    width: u16,
    height: u16,
    id_drawable: u32,
    id_gc: u32,
    // also XftColor *scheme and Fnt *fonts
}

impl<'a> Draw<'a> {
    pub fn new(conn: &RustConnection, screen: usize, root: u32, 
               width: u16, height: u16, depth: u8) -> Draw {
        // gen drawable
        let id_drw = conn.generate_id().expect("Draw::new: id_drw failed");
        let cookie_drw = create_pixmap(conn, depth, id_drw, root, width, height)
            .expect("Draw::new: create_pixmap failed");
        
        // gen graphics context
        let id_gcontext = conn.generate_id().expect("Draw::new: id_gcontext failed");
        let cookie_gcontext = create_gc(conn, id_gcontext, root, &CreateGCAux::new())
            .expect("Draw::new: create_gc failed");

        Draw {
            conn,
            screen,
            root,
            width,
            height,
            id_drawable: id_drw,
            id_gc: id_gcontext,
            
        }
    }

    fn resize(&mut self, width: u16, height: u16, depth: u8) {
        self.width = width;
        self.height = height;

        // free old pixmap and make anew
        let _cookie_fdrw = free_pixmap(self.conn, self.id_drawable)
            .expect("Drw::resize: free_pixmap failed");
        let id_drawable = self.conn.generate_id().expect("Drw::resize: id_drawable failed");
        let _cookie_drw = create_pixmap(self.conn, depth, id_drawable, self.root, width, height)
            .expect("Drw::resize: create_pixmap failed");

        self.id_drawable = id_drawable;
    }

    fn create_color() {}

    fn draw_rect(&self, x: usize, y: usize, w: usize, h: usize,
                     filled: bool, invert: bool) { todo!(); }
}

impl<'a> Drop for Draw<'a> {
    fn drop(&mut self) {
        free_gc(self.conn, self.id_gc).expect("Draw::drop: free_gc failed");
        free_pixmap(self.conn, self.id_drawable).expect("Draw::drop: free_pixmap failed");
    }
}
