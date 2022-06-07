// Modules
// mod structs;
// mod tagmasks;
mod event_handlers;
mod enums;
mod consts;
// mod drawable;
// mod config;
// mod types;
// mod wrapx;

// Usings
use std::env;
use die::die;
use openbsd::pledge;

/* X11 */
// use x11::xlib::*;
// use x11::keysym::*;
// use x11::*;
use x11rb::{self, COPY_DEPTH_FROM_PARENT};
use x11rb::connection::Connection;
use x11rb::protocol::Event;
use x11rb::protocol::randr::{MonitorInfo, get_screen_info};
use x11rb::protocol::xproto::{Atom, Screen, ConnectionExt, WindowClass, CreateWindowAux};
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
fn update_status(text: &str) {
    
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
    let screen_width = screen.width_in_pixels;
    let screen_height = screen.height_in_pixels;
    let root_window = screen.root;

    // TODO: add drawable etc. here. Also consider how globals etc are to 
    // work; env parameter passed about might still be a good idea.


    // let screen = DefaultScreen(display); /* this is returned from ::connect and stored in
    // screen_num 
    // let screen_width = DisplayWidth(display, screen);

    // let screen_height = DisplayHeight(display, screen);
    // let root = RootWindow(display, screen);
    // let drawable = drawable::MyDrawable::new(display, screen, root, screen_width, screen_height); // TODO: replace with Drw::new

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

fn handle_event(event: Event, env: &Environment) {
    use event_handlers::*;
    // TODO: a lot of these event handlers need global state in dwm. How should 
    // I do that here? I could pass in a variable (environment) which I mutate
    // in these... but this is global state. I want to make this more pure. 
    // TODO: add environment into here
    
    match event {
        // Event::ButtonPress(e) => button_press(e, env),
        // Event::ClientMessage(e) => client_message(e, env),
        // Event::ConfigureRequest(e) => configure_request(e, env),
        // Event::ConfigureNotify(e) => configure_notify(e, env),
        // Event::DestroyNotify(e) => destroy_notify(e, env),
        // Event::EnterNotify(e) => enter_notify(e, env),
        // Event::Expose(e) => expose(e, env), 
        // Event::FocusIn(e) => focus_in(e, env),
        // Event::KeyPress(e) => key_press(e, env),
        // Event::MappingNotify(e) => mapping_notify(e, env),
        // Event::MapRequest(e) => map_request(e, env),
        // Event::MotionNotify(e) => motion_notify(e, env),
        // Event::PropertyNotify(e) => property_notify(e, env),
        // Event::UnmapNotify(e) => unmap_notify(e, env),
        _ => {}, // do nothing 
    };
}

fn run(conn: &RustConnection, env: &Environment) {
    let running = true; 

    loop {
        // TODO: this is just here to start, this needs to be double checked
        if !running { break; }

        if let Ok(event) = conn.wait_for_event() {
            handle_event(event, env);
        }
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

    // TODO: do not believe this to be correct. 
    // let (sw, sh) = x11rb::protocol::randr::get_screen_info(conn, screen_num);
    let screen = &conn.setup().roots[screen_num];
    let win_id = conn.generate_id()?;

    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id, 
        screen.root, 
        0, 0, 
        100, 100, 
        0, WindowClass::INPUT_OUTPUT, 
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel)
        );
    
    // let env = Environment {
    //     status_text: "",
    //     screen_width: 0,
    // };
    // todo: rest of main
    // check_other_wm(conn); // TODO: implement
    setup(&conn, screen_num);
    // scan();
    // run();
    // cleanup();
    
    Ok(())

}
