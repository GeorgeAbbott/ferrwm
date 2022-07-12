#[allow(dead_code)]
pub struct Xy {
    x: i32, 
    y: i32,
}

impl Xy {
    #[allow(dead_code)]
    pub fn new(x: i32, y: i32) -> Xy {
        Xy {
            x,
            y,
        }
        
    }
}

impl Default for Xy {
    fn default() -> Xy {
        Xy {
            x: 0,
            y: 0,
        }
    }
}

#[allow(dead_code)]
pub struct Wh {
    w: i32,
    h: i32,
}

impl Wh {
    #[allow(dead_code)]
    pub fn new(w: i32, h: i32) -> Wh {
        Wh {
            w, 
            h,
        }
    }
}

impl Default for Wh {
    fn default() -> Self {
        Wh {
            w: 0,
            h: 0,
        }
    }
}

pub struct Rect {
    pub x: u32, 
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new_zeroed() -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }

    #[allow(dead_code)]
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect {
            x, y, w, h, 
        }
    }

    #[allow(dead_code)]
    pub fn is_zeroed(&self) -> bool {
        if self.x == 0 && self.y == 0 && self.w == 0 && self.h == 0 {
            true
        } else {
            false
        }
    }
}
