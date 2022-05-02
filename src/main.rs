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
use openbsd::pledge;

/* X11 */
// use x11::xlib::*;
// use x11::keysym::*;
// use x11::*;
use x11rb;
use x11rb::protocol::xproto;
use x11rb::rust_connection::RustConnection;


/* Wrapper Types */
use crate::types::display::Display;
use crate::types::fnt::Fnt;
use crate::types::drw::Drw;
use crate::config;
use crate::xwrap::supports_locale;



fn setup(conn: &RustConnection) {
    // let window_attributes: x11::xlib::XSetWindowAttributes;
    // let atom: x11::xlib::Atom;
    


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
    } else if argc != 1 {
        die!("usage: riodwm [-v]");
    }
    // TODO: add error checking for locale

    if cfg!(openbsd) && pledge!("stdio rpath proc exec").is_err() {
        die!("riodwm: pledge");
    }
    

    let (conn, screen_num) = x11rb::connect(None).unwrap();
    // TODO: do not believe this to be correct. 
    let (sw, sh) = x11rb::protocol::randr::get_screen_info(conn, screen_num);

    check_other_wm(conn); // TODO: implement
    setup(conn);
    scan();
    run();
    // cleanup();

}
