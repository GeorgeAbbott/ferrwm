pub struct Client {
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
        if !XGetWMNormalHints(dpy, c->win, &size, &msize) {
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


