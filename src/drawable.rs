use x11::{xft::XftColor, xlib::{Drawable, XCreatePixmap, XCreateGC, XSetLineAttributes, LineSolid, CapButt, JoinMiter, XFreePixmap}};

// TODO: I reckon this is unnecessary - is typedef struct { Cursor c; } Cur; 
// in C equivalent.
pub struct Cursor { 
    Cursor: cursor,
}

impl Cursor {
    pub fn new(drawable: &Drawable, shape: i32) -> Cursor {
        // TODO: change type i32 shape into its own enum 
        todo()!;
    }
}


pub struct Font { // C name: Fnt
    display: Display,
    height: usize,
    XftFont *xfont,     // TODO
    FcPattern *pattern, // TODO
    struct Fnt *next    // TODO ??? not sure what this means
}

impl Font {
    /// From C dwm drw_fontset_create function. 
    /// Final param fontcount not needed as fonts.len() can be used.
    /// C version can return a null pointer if not created, but returning an 
    /// Option<> here is more Rust-ic.
    pub fn new(drawable: &MyDrawable, fonts: Vec<&str>) -> Option<Font> {
        todo!();

        let current: Option<Font> = None;
        let ret: Option<Font> = None;
        let i: usize = 0;
    }

    /// From C dwm drw_fontset_getwidth function.
    fn width(&self, text: &str) -> u32 {
        todo!();
    }

    // TODO: figure out what this is. C dwm name is drw_fontset_getexts.
    // Params: Fnt *font, const char *text, unsigned int len, unsigned int *w, unsigned int *h
    // what strikes me is mutable access to Fnt (hence &mut self), len as 
    // unsigned int not usize_t, and mutable pointers to w and h 
    fn get_exts(&mut self, text: &str, u32 len, width: u32, height: u32) {
        todo!();
    }
}

enum ColorSchemeIndex {
    Foreground,
    Background,
    Border,
}

type Color = XftColor; // TODO; C name: Clr

/// Name is temporary, to avoid collision with x11::xlib::Drawable.
/// colorscheme and fonts members are optional as they are not set in 
/// the drw_create function in C.
pub struct MyDrawable {
    width: u32,
    height: u32,
    display: *Display, // TODO
    screen: i32,       // see impl block
    root: Window, // TODO
    drawable: x11::xlib::Drawable, // TODO make sure no name clash
    gc: GC, // TODO: what is this?
    colorscheme: Option<Color>, // TODO is ptr in C
    fonts: Option<Font>, // TODO: is ptr in C
}

impl MyDrawable {
    // TODO: Display and Window params need figuring out.
    // Also, why is screen an i32? 
    pub fn new(display: &Display, 
               screen: i32, 
               root: Window,
               window: &Window, 
               width: u32, 
               height: u32) -> MyDrawable {
        let ret = MyDrawable {
            display: display,
            screen: screen,
            root: root,
            width: width,
            height: height,
            drawable: unsafe {
                XCreatePixmap(display, root, width, height, DefaultDepth(display, screen))
            },
            gc: unsafe { XCreateGC(display, root, 0, 0) }, // last param is NULL in C version
            colorscheme: None,
            fonts: None,
        };
        unsafe {
            XSetLineAttributes(display, drawable.gc, 1, LineSolid, CapButt, JoinMiter);
        }

        ret
    }

    /// Equivalent of C function drw_resize in drw.c.
    fn resize(&mut self, width: u32, height: u32) {
        /* C version returns if (!drw) - I think we can miss this as we know
         * it is not null if this function is called.
         */
        
        self.width = width;
        self.height = height;

        // Clear drawable (likely needs freeing) and replace it
        XFreePixmap(self.display, self.drawable);
        self.drawable = XCreatePixmap() // TODO: fill out the parameters here
        



    }
}








