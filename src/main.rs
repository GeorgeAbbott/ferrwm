#![allow(unused_variables)]
// Modules
mod bar;
mod client;
mod config;
mod consts;
mod drawable;
mod enums;
mod geom;
mod monitor;
mod tag;
mod utils;
mod wm;

// Usings
use die::die;
use openbsd::pledge;
use std::env;

// crate
use drawable::Draw;
use utils::logf;
use wm::WindowManager;

// X11
use x11rb::connection::Connection;
use x11rb::protocol::xproto::ChangeWindowAttributesAux;
use x11rb::protocol::xproto::{change_window_attributes, EventMask};
use x11rb::rust_connection::RustConnection;

#[allow(dead_code)]
struct Environment {
    pub status_text: String, // TODO: check ownership: String or &'a str?
    pub screen: i32,
    pub screen_width: i32,
    pub screen_height: i32,
    pub bar_height: i32,
    pub bar_lw: i32,     // TODO: what is blw in original?
    pub lr_padding: i32, // sum of left and right padding for text
    // xerrorxlib - ???
    pub numlock_mask: i32,
}

fn setup(conn: &RustConnection, screen_num: usize) {
    /* init screen */
    let screen = &conn.setup().roots[screen_num];
    let screen_width: u16 = screen.width_in_pixels;
    let screen_height: u16 = screen.height_in_pixels;
    let root: u32 = screen.root;
    let screen_depth: u8 = screen.root_depth;

    let draw = Draw::new(
        conn,
        screen_num,
        root,
        screen_width,
        screen_height,
        screen_depth,
    );
    // let fontset_create_result = draw.create_fontset(fonts);

    let mask = EventMask::SUBSTRUCTURE_REDIRECT
        | EventMask::SUBSTRUCTURE_NOTIFY
        | EventMask::BUTTON_PRESS
        | EventMask::POINTER_MOTION
        | EventMask::ENTER_WINDOW
        | EventMask::LEAVE_WINDOW
        | EventMask::STRUCTURE_NOTIFY
        | EventMask::PROPERTY_CHANGE
        | EventMask::KEY_PRESS;
    let cwa = ChangeWindowAttributesAux::new().event_mask(mask);

    if let Some(err) = change_window_attributes(conn, root, &cwa)
        .expect("setup: change_window_attributes failed")
        .check()
        .err()
    {
        die!("Failed with error {}", err.to_string());
    }

    let mut wm = WindowManager::new(conn, screen_num);
    wm.run_event_loop();
}

#[allow(dead_code)]
fn scan() {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logf("ferrwm started");
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    if argc == 2 && argv[1] != "-v" {
        die!("ferrwm-{}", consts::VERSION);
    } else if argc != 1 {
        die!("usage: ferrwm [-v]");
    }
    // TODO: add error checking for locale

    if cfg!(openbsd) && pledge!("stdio rpath proc exec").is_err() {
        die!("ferrwm: pledge");
    }

    let (conn, screen_num) = x11rb::connect(None).unwrap();

    setup(&conn, screen_num);
    // scan();
    // cleanup();

    Ok(())
}
