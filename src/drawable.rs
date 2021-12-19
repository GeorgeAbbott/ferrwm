use x11::{xft::XftColor, xlib::Drawable};

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
    }

    /// From C dwm drw_fontset_getwidth function.
    // TODO: consider using usize not u32? Perhaps better for width.
    fn width(&self, text: &str) -> u32 {
        todo!();
    }

    // TODO: figure out what this is. C dwm name is drw_fontset_getexts.
    // Params: Fnt *font, const char *text, unsigned int len, unsigned int *w, unsigned int *h
    // what strikes me is mutable access to Fnt (hence &mut self), len as 
    // unsigned int not usize_t, and mutable pointers to w and h 
    // I have changed width / height from u32 to usize for consistency
    fn get_exts(&mut self, text: &str, u32 len, width: usize, height: usize) {
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
pub struct MyDrawable {
    width: usize,
    height: usize,
    display: *Display, // TODO
    screen: i32,       // see impl block
    root: Window, // TODO
    drawable: x11::xlib::Drawable, // TODO make sure no name clash
    gc: GC, // TODO: what is this?
    colorscheme: Color, // TODO is ptr in C
    fonts: Font, // TODO: is ptr in C
}

impl MyDrawable {
    // TODO: Display and Window params need figuring out.
    // Also, why is screen an i32? 
    pub fn new(display: &Display, 
               screen: i32, 
               window: &Window, 
               width: usize, 
               height: usize) -> MyDrawable {

        todo!();
    }
}








