use std::fs::OpenOptions;
use std::io::Write;

use x11rb::protocol::xproto::{Keycode, KeyButMask};

pub enum Key {
    Q,
    W,
}

pub fn logf(string: &str) {
    let mut file = OpenOptions::new().append(true).open("/var/log/ferrwm.log")
        .expect("Could not open logfile");
    file.write_all(string.as_bytes()).expect("Log write failed");
    file.write_all("\n".as_bytes()).expect("Newline write failed");
}

// Given a particular keypress and keystate, and the desired mask and keycode,
// (I will soon hope to make these keysyms...) find if a combination has been
// pressed.
pub fn is_pressed(
                  state: (u8, u16), 
                  desired: u8, 
                  mask: KeyButMask) -> bool {
    let mask = u16::from(mask);
    let keypress = state.0;
    let keystate = state.1;

    keypress == desired && keystate == mask
}

// Takes the enum that is in config file, and converts it to a usable
// X Keycode. But of course, I want to be working with keysyms...
pub fn configkey_to_key(key: &Key) -> u8 {
    match key {
        Key::Q => 24,
        Key::W => 25,
    }

}
