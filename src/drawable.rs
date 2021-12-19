// TODO: I reckon this is unnecessary - is typedef struct { Cursor c; } Cur; 
// in C equivalent.
pub struct Cursor { 
    Cursor: cursor,
}

pub struct Font { // C name: Fnt
    display: Display,
    height: usize,
    XftFont *xfont,     // TODO
    FcPattern *pattern, // TODO
    struct Fnt *next    // TODO ??? not sure what this means
}




