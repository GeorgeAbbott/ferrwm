use std::fs::OpenOptions;
use std::io::Write;

use die::die;
use x11rb::protocol::xproto::{Keycode, KeyButMask};

pub enum Key {
    Invalid,
    Q,
    W,
    E,
}

pub fn logf(string: &str) {
    let mut file = OpenOptions::new().append(true).open("/var/log/ferrwm.log")
        .expect("Could not open logfile");
    file.write_all(string.as_bytes()).expect("Log write failed");
    file.write_all("\n".as_bytes()).expect("Newline write failed");
}

pub fn key_to_cfgkey(key: Keycode) -> Key {
    match key {
        24 => Key::Q,
        25 => Key::W,
        26 => Key::E,
        _ =>  Key::Invalid,
    }
}
