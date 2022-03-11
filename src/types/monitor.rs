struct Monitor {
    ltsymbol: String,
    mfact: f32,
    nmaster: i32,
    num: i32,
    bar_y: i32,
    mon_x, mon_y, mon_w, mon_h: i32, // TODO: can I do multiple declarations 
                                     // like this?
    win_x, win_y, win_w, win_h: i32,
    seltags: u32,
    sellt: u32,
    tagset: [u32; 2], // TODO: correct syntax?
    show_bar: bool,
    top_bar: bool,
    clients: &Client, // TODO: what if I need mut access? find out if do
    sel: &Client,
    stack: &Client,
    next: &Monitor, // TODO: believe may actually be null, so change to 
                    //  Option<> as and when 
    barwin: Window,
}

impl Monitor {
    fn new() -> Self {
        todo!();
    }

    // Equivalent of dwm.c#updatebarpos.
    fn update_bar_position(&mut self, bar_height: i32) {
        self.win_y = self.mon_y;
        self.win_h = self.mon_h;

        if self.show_bar {
            m.win_h -= bar_height;

            let new_bar_y = if self.top_bar { self.win_y } 
                            else { self.win_y + self.win_h };

            let new_win_y = if self.top_bar { self.win_y + bar_height }
                            else { self.win_y };

            self.bar_y = new_bar_y;
            self.win_y = new_win_y;
        } else {
            self.bar_y = -bar_height;
        }
    }







}
