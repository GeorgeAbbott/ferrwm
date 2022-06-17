// Temp file literally just for work in progress stuff.

struct Tags {
    pub inner: i32,
}


// Represents an individual tag. Stores a linked list of references to each 
// window client on that tag. WindowTag instances are all managed by 
// WindowManager. 
struct WindowTag<'a> {
    windows: LinkedList<&'a ClientWindow>,
}

impl<'a> WindowTag<'a> {
    pub fn new() -> Self { 
        Self {
            windows: LinkedList::new(),
        }
    }

    /// Assign a client to this.
    pub fn assign_client(client: &ClientWindow) {

    }
}

struct Xy(i32, i32);
struct Wh(i32, i32);


struct ClientWindow {
    name: String,
    minmax_aspect: (f32, f32),
    xy: Xy,
    wh: Wh,
    old_xy: Xy,
    old_wh: Wh,
    base_wh: Wh, 
    inc_wh: Wh,
    max_wh: Wh,
    min_wh: Wh,
    border_width: i32,
    old_border_width: i32,
    tags: Tags,
    is_fixed: bool,
    is_floating: bool, 
    is_urgent: bool, 
    never_focus: bool,
    is_fullscreen: bool, 


}
