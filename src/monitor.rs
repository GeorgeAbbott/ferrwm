use x11rb::protocol::xproto::Screen;
use x11rb::rust_connection::RustConnection;

use crate::enums::BarPosition;
use crate::geom::Rect;
use crate::bar::Bar;
use crate::client::Client;
use crate::tag::Tags;
use crate::config::HIDE_BAR;
use crate::config::BAR_POSITION;

// Represents a monitor, which owns a set of clients. When clients move from 
// one monitor to another, at least for now this is done by WindowManager.
pub struct Monitor<'rc> {
    id: i32,
    pub bar: Bar<'rc>, 
    rect: Rect,
    selected_tags: Tags, 
    bar_hidden: bool,
    bar_pos: BarPosition,

    clients: Vec<Client<'rc>>,
    selected_client: usize,  // index into clients
    // ... rest TBA
}

impl<'rc> Monitor<'rc> {
    pub fn new(conn: &'rc RustConnection, screen: &Screen) -> Self {
        let id = 0; // Only generated if with Xinerama I think? 
        let bar = Bar::new(conn, screen);
        let tags = Tags::new();
        tags.set(0); // set the first tag upon startup 

        Self {
            id, 
            bar,
            rect: Rect::new_zeroed(), // TODO: find. this is x, y, w, h in dwm
            selected_tags: tags,
            bar_hidden: HIDE_BAR,
            bar_pos: BAR_POSITION,
            clients: Vec::new(),
            selected_client: 0, // TODO: find
        }
    }

    /// TODO: figure out what I actually want to do with this. Where should 
    /// stext be held... 
    pub fn set_stext(&mut self, text: &str) {
    }

    /// Draw the status bar. For my future self, this should be here as it 
    /// needs access to monitor's Vec of Clients. 
    pub fn draw_bar(&self) { todo!(); }

    pub fn add_client(&mut self, c: Client) {
        self.clients.push(c);
    }

    #[allow(dead_code)]
    pub fn toggle_bar(&mut self) { self.bar_hidden = !self.bar_hidden; }

    #[allow(dead_code)]
    pub fn hide_bar(&mut self) { self.bar_hidden = true; }

    #[allow(dead_code)]
    pub fn show_bar(&mut self) { self.bar_hidden = false; }

    #[allow(dead_code)]
    pub fn get_sel_client(&self) -> &Client {
        &self.clients[self.selected_client]
    }

    #[allow(dead_code)]
    pub fn get_sel_client_mut(&mut self) -> &mut Client {
        &mut self.clients[self.selected_client]
    }
}
