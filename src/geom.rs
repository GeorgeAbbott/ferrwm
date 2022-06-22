pub struct Xy {
    x: i32, 
    y: i32,
}

impl Xy {
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

pub struct Wh {
    w: i32,
    h: i32,
}

impl Wh {
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
