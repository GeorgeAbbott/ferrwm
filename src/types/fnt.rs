pub struct Fnt {
    display: &Display, // this sure does feel like a multiple mut ref....
    height: u32,
    xfont: XftFont, // does this need ptr to this?
    pattern: FcPattern, // ^^ 
    font: Box<Fnt>, // consider other allocation means, e.g. raw ptr or ref
}

impl Fnt {
    pub fn new() -> Self {
        Self {
            todo!();
        }
    }

    fn get_exts(&self, text: &str) -> (u32, u32) {
        // create uninitialised XGlyphInfo ext here
        
        todo!();
    }
}

impl Drop for Fnt {
    fn drop(&mut self) {
        unsafe {
            // free stuff here 
        }
    }
}

