// For config file
pub enum BarPosition {
    Top,
    Bottom,
}

pub enum Click {
    TagBar,
    LtSymbol, // ???
    StatusText,
    WindowTitle,
    ClientWindow,
    RootWindow,
    Last,
}

// For dwm.c equiv
enum Cursor {
    // TODO: this is just used as an alias for list indices so maybe even 
    // just dont bother
    // I dont really want to use a hashmap for these lists though, perhaps 
    // find alias for indices and just use that?
}


