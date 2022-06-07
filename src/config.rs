/* All configurations can be changed here to fit your preferences. */
use crate::enums::{BarPosition};


// Appearance
pub const BORDERPX: i32             = 1;    /* border pixel of windows */
pub const GAPPX: i32                = 24;   /* gaps between windows */
pub const SNAP: i32                 = 32;   /* snap pixel */
pub const SHOW_BAR: bool            = true;
pub const BAR_POSITION: BarPosition = BarPosition::Top;

pub const FONTS: &[&str]            = &["Liberation Mono:size=11"];
pub const DMENU_FONT: &str          = "Liberation Mono;size=11";

pub const NUM_COLOR: usize          = 3; // DO NOT CHANGE
pub const COLORS: &[&[&str; NUM_COLOR]] =
    [    /* foreground   background   border    */
         [  "#ffffff",   "#000000",   "#000000"  ],
         // TODO add further elements... 
    ];

pub const TAGS: &[&str] = &[ "1", "2", "3", "4", "5", "6", "7", "8", "9", ];
                            















