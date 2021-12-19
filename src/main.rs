// Modules
mod structs;
mod tagmasks;
mod enums;
mod consts;
mod drawable;

// Usings
use std::env;
use die::die;

/* X11 */
use x11::xlib::*;
use x11::keysym::*;
use x11::*;

fn check_other_wm() {
}

fn setup() {
    let window_attributes: x11::xlib::XSetWindowAttributes;
    let atom: x11::xlib::Atom;

    /* clean up any zombies immediately */
    sigchld(0); // TODO: ???

    /* init screen */
    let screen = DefaultScreen(display);
    let screen_width = DisplayWidth(display, screen);
    let screen_height = DisplayHeight(display, screen);
    let root = RootWindow(display, screen);
    let drawable = create_drawable(display, screen, root, screen_width, screen_height); // drw_create

    if !create_drawable_fontset(drawable, fonts, fonts.len()) {
        die!("no fonts could be loaded.");
    }

    let lrpad = drawable->fonts->height; // TODO

    let bh = drawable->fonts->height + 2; // TODO
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
    if !set_locale(LC_CTYPE, "") || !XSupportsLocale() {
        todo!();
    }

    #[cfg(target_os="openbsd")]
    {
        display = open_display(NULL); // TODO: use C NULL
        if display.is_none() { // TODO: figure out 
            die!("dwm: cannot open display");
        }
    }




    // todo: rest of main
    check_other_wm();
    setup();
    scan();
    run();
    // cleanup();
    unsafe {
        XCloseDisplay(display);
    }



}
