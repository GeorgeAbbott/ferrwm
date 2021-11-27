// Modules
mod structs;
mod tagmasks;
mod enums;
mod consts;

// Usings
use std::env;
use die::die;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    
    if argc == 2 && argv[1] != "-v" {
        die!("riodwm-{}", consts::VERSION);
    }
    else if argc != 1 {
        die!("usage: dwm [-v]");
    }

    // todo: rest of main

}
