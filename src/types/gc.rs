use x11::xlib::{XCreateGC, XFreeGC, GC}; 
// NOTE: consider DO NOT USE. This is just here for reference now, but GC is 
// probably better off being owned by Drw as this is the only place it is used.
struct Gc {
    inner: _XGC,
}

impl Gc {
    // TODO: 
    // 1. confirm it is fine to pass dpy by ref as I can call as_raw
    // 2. pass Window by ref or not?
    pub fn new(dpy: &Display, root: Window) -> Self {
        unsafe { 
            Self {
                inner: XCreateGC(dpy.as_raw(), root, 0, NULL),
            }
        }
    }
}

impl Drop for Gc {
    fn drop(&mut self) {
        
    }




