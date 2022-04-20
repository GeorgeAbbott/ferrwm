use x11::xlib::{XGetWMNormalHints, XSizeHints};

pub struct Client {
    name: String, // TODO: double check should be owned
    min_a: f32, 
    max_a: f32, 
    xy: (i32, i32),
    wh: (i32, i32),
    oldxy: (i32, i32),
    oldwh: (i32, i32),
    basewh: (i32, i32),
    incwh: (i32, i32),
    maxwh: (i32, i32),
    minwh: (i32, i32), 
    bw: i32, oldbw: i32, // TODO: what is bw?
    tags: u32, // TODO: Potentially choose something cleaner as bitwise is ugly.
    
    // Statuses
    isfixed: bool,
    isfloating: bool,
    isurgent: bool,
    neverfocus: bool,
    oldstate: bool // TODO: is this actually a bool? Check dwm prev versions to confirm.
    isfullscreen: bool,

    next: Box<Client>,
    snext: Box<Client>, // TODO: what is snext?

    monitor: Monitor,  // Monitor* type, should I use ref for this? 
    window: Window,   // Window type, not ptr in C



}

impl Client {
    pub fn new() -> Self {
        Self {
        }
    }

    fn update_size_hints(&mut self) {
        let msize: i64 = -1;
        let size:  Option<XSizeHints> = None; // TODO: maybe just make raw ptr?

        // just dip into unsafe for this, also not sure what return type is
        // is it true/false or int or a pointer that we are checking for null?
        if !unsafe { XGetWMNormalHints(dpy, c->win, &size, &msize); } { 
            // TODO: check return of this line as this might be a null check
            // I really just surrounded it with unsafe for it to stop erroring
            // for now.
            todo!();
        }
    }

    // Not sure if this works, similar to C code
    // reckon it should be a method here though
    fn attach(&mut self) {
        self.next = self.monitor.clients;
        self.monitor.clients = self;
    }

    fn attach_stack(&mut self) {
        self.snext - self.monitor.stack;
        self.monitor.stack = c;


