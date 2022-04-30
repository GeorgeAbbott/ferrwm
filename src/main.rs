// Modules
// mod structs;
// mod tagmasks;
mod enums;
mod consts;
mod drawable;
mod config;
mod types;
mod wrapx;

// Usings
use std::env;
use die::die;

/* X11 */
use x11::xlib::*;
use x11::keysym::*;
use x11::*;

/* Wrapper Types */
use crate::types::display::Display;
use crate::types::fnt::Fnt;
use crate::types::drw::Drw;
use crate::config;
use crate::xwrap::{supports_locale};


fn check_other_wm() {
}

fn setup(display: &Display) {
    let window_attributes: x11::xlib::XSetWindowAttributes;
    let atom: x11::xlib::Atom;

    /* clean up any zombies immediately */
    // sigchld(0); // TODO: ???

    /* init screen */
    let screen = DefaultScreen(display);
    let screen_width = DisplayWidth(display, screen);
    let screen_height = DisplayHeight(display, screen);
    let root = RootWindow(display, screen);
    let drawable = drawable::MyDrawable::new(display, screen, root, screen_width, screen_height); // TODO: replace with Drw::new

    let font = drawable::Font::new(&drawable, 
                                   config::FONTS); // TODO: replace w/ Fnt::new
    if font.is_none() {
        die!("no fonts could be loaded.");
    } 
    let font = font.unwrap();


    let lrpad = drawable.fonts.height; // TODO: make these public

    let bar_height = drawable.fonts.height + 2; // will not make global but instead pass to every func that needs it
    updategeom(); // TODO
                  

    /* init atoms */
    todo!();

}

fn scan() {
}

fn run() {
}




fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    
    if argc == 2 && argv[1] != "-v" {
        die!("riodwm-{}", consts::VERSION);
    }
    else if argc != 1 {
        die!("usage: dwm [-v]");
    }

    // Error checking
    if !set_locale(LC_CTYPE, "") || !(supports_locale()) { // TODO: set_locale
        todo!();
    }

    // if cfg!(openbsd) && pledge("stdio rpath proc exec", NULL) == -1 {
    //     die!("pledge");
    // }



    // Wrapper type: should not need to manage mem as should
    // be dealt with by impl
    let display = Display::new(); // What is the error about _XDisplay?
    if display.is_none() { 
        die!("dwm: cannot open display");
    }




    // todo: rest of main
    check_other_wm();
    setup(display);
    scan();
    run();
    // cleanup();

}
