use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn logf(string: &str) {
    let mut file = OpenOptions::new().append(true).open("/var/log/ferrwm.log")
        .expect("Could not open logfile");
    file.write_all(string.as_bytes()).expect("Log write failed");
    file.write_all("\n".as_bytes()).expect("Newline write failed");
}
