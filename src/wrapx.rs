// Wraps several X functions just so can I can call them without having to 
// clutter the actual codebase with unsafes.

use x11::xlib::{XSupportsLocale};

pub fn supports_locale() -> bool {
    unsafe {
        XSupportsLocale() as bool
    }
}




