#![allow(unused_variables)]
#![allow(dead_code)]
use x11rb::protocol::xproto::{KeyButMask, Keysym, Keycode};

/* All configurations can be changed here to fit your preferences. */
use crate::{enums::{BarPosition}, utils::{Key, logf, key_to_cfgkey}};
use crate::wm::WindowManager;


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
pub enum Argument {
    Str(&'static str),
    Int(i32),
}

// Keybindings.
impl<'wm, 'rc> WindowManager<'wm, 'rc> {
    pub fn act_on_keypress(&mut self, mask: KeyButMask, key: Keycode) {
        let ctrl  = KeyButMask::CONTROL;
        let shift = KeyButMask::SHIFT;
        let meta  = KeyButMask::MOD1;
        let shift_ctrl = KeyButMask::CONTROL | KeyButMask::SHIFT;
        let shift_meta = KeyButMask::MOD1    | KeyButMask::SHIFT;

        // NOTE: this name is actually wrong. This is not a keysym, I just 
        // want it to be in the future when I get keysyms working. I think.
        let keysym = key_to_cfgkey(key); 

        match (mask, keysym) {
            // Add in keybindings here.
            // FIXME: This is being called on any keypress of Q. Why?
            (shift_meta, Key::Q) => self.quit(),
            // FIXME: why is this an unreachable pattern? I believe this should
            // match just fine.
            (ctrl, Key::Q)  => testfn(&Argument::Str("Q")),
            (ctrl, Key::W)  => testfn(&Argument::Str("W")),
            (ctrl, Key::E)  => self.spawn_window("dmenu"),
            _ => logf("uhoh, an unrecognized keypress"),
        }
    }
}

// spawn, togglebar, focusstack, incnmaster, setmfact, zoom, view, 
// killclient, setlayout, togglefloating, tag, focusmon, tagmon, quit.
