// TODO: sort out the names here of types
pub struct Drw {
    width: u32,
    height: u32,
    display: &Display,
    screen: i32,
    drawable: Option<Drawable>, // TODO: ref to?
    gc: GC, // ???
    scheme: &Clr,
    fonts: &Fnt,
}

impl Drw {
    pub fn new(display: &Display, 
               screen:  i32,
               root:    Window,
               width:   u32,
               height:  u32) -> Self {
        todo!();
        // This will need to call ecalloc so maybe it needs to 
        // be boxed? Also, XCreatePixmap, XCreateGC, XSetLineAttributes
        // will need to be called. 
    }

    fn resize(&mut self, width: u32, height: u32) {
        todo!();
        self.width = width;
        self.height = height;

        if let Some(d) = self.drawable { // TOCHECK: correct use of if let?
            // XFreePixmap(drw->dpy, drw->drawable);
            todo!();
        }

        // TODO: make Drawable type
        // Since we own self.root, should we clone it? is it a large type?
        // How do we handle DefaultDepth function?
        self.drawable = Some(Drawable::new(&self.display, self.root, 
                width, height, DefaultDepth(todo!())));
    }
}

impl Drop for Drw {
    fn drop(&mut self) {
        todo!();
    }
} 



