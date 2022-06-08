use x11rb::{rust_connection::RustConnection, protocol::xproto::{Window, Drawable, GC, create_pixmap, create_gc, CreateGCAux}, connection::Connection};



pub struct Draw<'a> {
    conn: &'a RustConnection,
    screen: usize,
    root: Window,
    width: usize,
    height: usize,
    drawable: Drawable,
    gc: GC
}

impl<'a> Draw<'a> {
    pub fn new(conn: &RustConnection, screen: usize, root: Window, 
               width: usize, height: usize) -> Self {
        
        let gcid = conn.generate_id();
        let gcaux = CreateGCAux::new();

        Self {
            conn,
            screen,
            root,
            width,
            height,
            drawable: create_pixmap(conn, depth, pid, drawable, width, height),
            gc: create_gc(conn, gcid, root, &gcaux)
            
        }
    }

    fn draw_rect(&self, x: usize, y: usize, w: usize, h: usize,
                     filled: bool, invert: bool) { todo!(); }
}
