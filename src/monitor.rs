use crate::enums::BarPosition;
use crate::geom::{Xy, Wh};
use crate::bar::Bar;
use crate::client::Client;

// Represents a monitor, which owns a set of clients. When clients move from 
// one monitor to another, at least for now this is done by WindowManager.
pub struct Monitor {
    id: i32,
    bar: Bar, 
    xy: Xy,
    wh: Wh,
    selected_tags: Tags, // TODO: make. Flags for tags.
    hide_bar: bool,
    bar_pos: BarPosition,
    clients: Vec<Client>,
    selected_client: usize,  // index into clients
    // ... rest TBA







}
