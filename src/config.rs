#![allow(unused_variables)]
#![allow(dead_code)]
use x11rb::protocol::xproto::KeyButMask;

/* All configurations can be changed here to fit your preferences. */
use crate::{enums::{BarPosition}, utils::{Key, logf}};


// Appearance
pub const BORDERPX: i32             = 1;    /* border pixel of windows */
pub const GAPPX: i32                = 24;   /* gaps between windows */
pub const SNAP: i32                 = 32;   /* snap pixel */
pub const HIDE_BAR: bool            = false;/* whether to by default hide bar */
pub const BAR_POSITION: BarPosition = BarPosition::Top;

pub const FONTS: &[&str]            = &["Liberation Mono:size=11"];
pub const DMENU_FONT: &str          = "Liberation Mono;size=11";

pub const NUM_COLOR: usize          = 3; // DO NOT CHANGE
// pub const COLORS: &[&[&str; NUM_COLOR]] =
//     [    /* foreground   background   border    */
//          [  "#ffffff",   "#000000",   "#000000"  ],
//          // TODO add further elements... 
//     ];

pub const TAGS: &[&str] = &[ "1", "2", "3", "4", "5", "6", "7", "8", "9", ];

pub fn testfn(arg: &Argument) {
    if let Argument::Str(s) = arg {
        logf(format!("You said {}", s).as_str());
    }
}

// Keybindings
pub struct Keybd(pub KeyButMask, pub Key, pub fn(&Argument) -> (), pub Argument);
pub enum Argument {
    Str(&'static str),
    Int(i32),
}
pub const KEYBINDINGS: &[Keybd] = &[
    Keybd(KeyButMask::SHIFT, Key::Q, testfn, Argument::Str("Q")),
    Keybd(KeyButMask::CONTROL, Key::W, testfn, Argument::Str("W")),
];




// spawn, togglebar, focusstack, incnmaster, setmfact, zoom, view, 
// killclient, setlayout, togglefloating, tag, focusmon, tagmon, quit.









