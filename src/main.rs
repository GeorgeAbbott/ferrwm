// Modules
// mod structs;
// mod tagmasks;
mod event_handlers;
mod enums;
mod consts;
mod drawable;
// mod config;
// mod types;
// mod wrapx;

// Usings
use std::env;
use die::die;
use openbsd::pledge;
use crate::drawable::Draw;
use event_handlers::*;

/* X11 */
// use x11::xlib::*;
// use x11::keysym::*;
// use x11::*;
use x11rb::{self, COPY_DEPTH_FROM_PARENT};
use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::randr::{MonitorInfo, get_screen_info};
use x11rb::protocol::xproto::{Atom, Screen, ConnectionExt, WindowClass, CreateWindowAux, CW, ChangeWindowAttributesAux, create_window, EventMask, change_property, change_window_attributes};
use x11rb::rust_connection::RustConnection;


/* Wrapper Types */
// use crate::types::display::Display;
// use crate::types::fnt::Fnt;
// use crate::types::drw::Drw;
// use crate::config;
// use crate::xwrap::supports_locale;

// Alias
type Monitor = MonitorInfo; // not sure whether monitor info correct struct 

// Update the status with the text if present, if empty use default text.
fn update_status(conn: &RustConnection, root_id: u32, text: &str) {
    // change_property(conn, root_id, )        
}

struct Environment {
    pub status_text: String,// TODO: check ownership: String or &'a str? 
    pub screen: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub bar_height: i32, 
    pub bar_lw: i32, // TODO: what is blw in original?
    pub lr_padding: i32, // sum of left and right padding for text
    // xerrorxlib - ???
    pub numlock_mask: i32,

}

fn setup(conn: &RustConnection, screen_num: usize) {
    // let window_attributes: x11::xlib::XSetWindowAttributes;
    // let atom: x11::xlib::Atom;
    /* clean up any zombies immediately */
    // sigchld(0); // TODO: ???
    /* init screen */
    let screen = &conn.setup().roots[screen_num];
    let screen_width: u16 = screen.width_in_pixels;
    let screen_height: u16 = screen.height_in_pixels;
    let root: u32 = screen.root;
    let screen_depth: u8 = screen.root_depth;

    let draw = Draw::new(conn, screen_num, root, screen_width, screen_height, screen_depth);
    // let fontset_create_result = draw.create_fontset(fonts);

    // TODO: find whatever the xcb equivalent for XSelectInput is, and use 
    // that to get SubstructureRedirectMask. 

    let mask =
        EventMask::SUBSTRUCTURE_REDIRECT |
        EventMask::SUBSTRUCTURE_NOTIFY; 

    let cookie_cwa = change_window_attributes(conn, root, 
                                        &ChangeWindowAttributesAux::new()
                                        .event_mask(mask))
        .expect("could not get substructure_redirect or substructure_notify");

    // let font = drawable::Font::new(&drawable, 
    //                                config::FONTS); // TODO: replace w/ Fnt::new
    // if font.is_none() {
    //     die!("no fonts could be loaded.");
    // } 
    // let font = font.unwrap();


    // let lrpad = drawable.fonts.height; // TODO: make these public

    // let bar_height = drawable.fonts.height + 2; // will not make global but instead pass to every func that needs it
    // // updategeom(); // TODO
    // 
    // /* init atoms */
    // todo!();

}

fn scan() {
}

fn handle_event(event: Event) {
    use event_handlers::*;
    // TODO: a lot of these event handlers need global state in dwm. How should 
    // I do that here? I could pass in a variable (environment) which I mutate
    // in these... but this is global state. I want to make this more pure. 
    // TODO: add environment into here
    
    match event {
        Event::ButtonPress(e) => button_press(e),
        Event::ClientMessage(e) => client_message(e),
        Event::ConfigureRequest(e) => configure_request(e),
        Event::ConfigureNotify(e) => configure_notify(e),
        Event::DestroyNotify(e) => destroy_notify(e),
        Event::EnterNotify(e) => enter_notify(e),
        Event::Expose(e) => expose(e),
        Event::FocusIn(e) => focus_in(e),
        Event::KeyPress(e) => key_press(e),
        Event::MappingNotify(e) => mapping_notify(e),
        Event::MapRequest(e) => map_request(e),
        Event::MotionNotify(e) => motion_notify(e),
        Event::PropertyNotify(e) => property_notify(e),
        Event::UnmapNotify(e) => unmap_notify(e),
        _ => {}, // do nothing 
    };
}

fn run(conn: &RustConnection) {
    while let Ok(event) = conn.wait_for_event() {
        handle_event(event);
    } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // check_other_wm(conn); // TODO: implement
    setup(&conn, screen_num);
    // scan();
    
    run(&conn);
    // cleanup();
    
    Ok(())

}
