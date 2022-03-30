// TODO: sort out the names here of types
pub struct Drw {
    width: u32,
    height: u32,
    display: &Display,
    screen: i32,
    drawable: Option<Drawable>, // TODO: ref to?
    gc: GC, // ???
    scheme: &Clr,
    fonts: &Fnt,
}


impl Drw {
    pub fn new(display: &Display, 
               screen:  i32,
               root:    Window,
               width:   u32,
               height:  u32) -> Self {
        todo!();
        // This will need to call ecalloc so maybe it needs to 
        // be boxed? Also, XCreatePixmap, XCreateGC, XSetLineAttributes
        // will need to be called. 
    }

    fn resize(&mut self, width: u32, height: u32) {
        todo!();
        self.width = width;
        self.height = height;

        if let Some(d) = self.drawable { // TOCHECK: correct use of if let?
            // XFreePixmap(drw->dpy, drw->drawable);
            todo!();
        }

        // TODO: make Drawable type
        // Since we own self.root, should we clone it? is it a large type?
        // How do we handle DefaultDepth function?
        self.drawable = Some(Drawable::new(&self.display, self.root, 
                width, height, DefaultDepth(todo!())));
    }

    // TODO: should this pass off onto Fnt::new in some way?
    // Equivalent to C dwm function: drw_fontset_create() in drw.c
    // TODO: change &Vec<String> to a better borrowed type?
    // Believe the third param, size_t fontcount not needed as size 
    // can be gotten
    // TODO: does this need to return *Fnt or &Fnt? cant it just assign the
    // font within the type? if so, can I call this in constructor?
    fn create_fontset(&mut self, fonts: &Vec<String>) -> Fnt {
        if fonts.size() == 0 { return; } // TODO: find better control flow

        todo!();
        // I believe here is reverse iterating over fonts and trying to 
        // call xfont_create on them? 
        
        // todo: figure out what that return statement here means
    }

    // See notes under ref dir. Not sure whether this should go here, 
    // like create_fontset(), but putting here for now.
    // Assuming by name that dest is the out param?
    // if so, consider making it return instead
    // TODO as per notes below move this into body of create_scheme
    fn create_color(&mut self, dest: &mut Clr, colorname: &str) {
        todo!();
    }
    
    // Returns an Option<Clr>, as C func can return null ref.
    // Mutability of this method needs only to be as much as 
    // create_color() - if this ends up w immut ref then this 
    // can take immut ref as well.
    // TODO: replace &Vec<> with borrowed type
    // TODO: not having clrcount is assuming that this is just colornames.len?
    //
    // NOTE: this function is only called in one place: dwm.c#setup(),
    // where it is called scheme[i] = drw_scm_create(drw, colors[i], 3);
    // hence, erroring on a colornames < 2 is never going to happen right? 
    // TODO: find whatever code in setup() that guarantees that size > 3 
    // so it can just be passed into the func so easily
    fn create_scheme(&mut self, colornames: &Vec<String>) -> Option<Clr> {
        if colornames.len() < 2 {
            return None; 
        }

        todo!(); 
        // TODO this part here creates using drw_clr_create, using ret and 
        // an index as an out param
        // hence, do we want dest as an out param or do we want to return it 
        // to have ownership of it? imo take ownership
        // also, if it creates with an index and a for loop, see where else 
        // drw_clr_create is called, if only here we might as well just put 
        // the looping code inside the create_color func altogether, and just
        // return a Vec<Clr>
        // maybe even just move func body into here to simplify
        // NOTE: I just checked a bit and afaict it is only called in this 
        // one function, so I guess I either make it priv method or just move 
        // it into this one 
        // probably easier to just move it into the body
    }

    fn set_fontset(&mut self, set: Fnt) {
        self.fonts = set;
    }

    fn set_scheme(&mut self, scm: Clr) { 
        self.scheme = scm;
    }

    // TODO: this function should be largely ok but might still
    // error for something
    fn rect(&self, x: i32, y: i32, 
        w: u32, h: u32, filled: bool, invert: bool) {

        // What to use 
        let col_ground = if invert { ColBg } else { ColFg };
        
        if self.scheme.is_none() // do I need a check like this?
                // how will I assert that drw->scheme is Some? 
                // will it be assigned at initialisation so always
                // be Some?
        { todo!(); }

        unsafe {
            XSetForeground(self.display, self.gc, 
                drw.scheme[col_ground].pixel);

            if filled {
                XFillRectangle(self.display, self.drawable, self.gc, 
                    x, y, w, h);
            } else {
                XDrawRectangle(self.display, self.drawable, self.gc,
                    x, y, w - 1, h - 1);
            }
        }
    }

    // TODO: C accepts Window as copy, but should we copy? Or 
    // #derive[Copy] for Window? Or allow to move / use reference?
    fn map(&self, win: Window, x: i32, y: i32, w: u32, h: u32) { 
        unsafe {
            XCopyArea(self.display, self.drawable, win, self.gc,
                x, y, w, h, x, y);

            // C call has False - is this a typedef int? Use 0 for now FIXME
            XSync(self.display, 0);
        }
    }


                




}

impl Drop for Drw {
    fn drop(&mut self) {
        todo!();
    }
} 



