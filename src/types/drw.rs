use x11::xlib::{XSync, XFillRectangle, XDrawRectangle};
use x11::xft::XftColorAllocName;
use crate::consts::{COLBG, COLFG, COLBORDER};
use std::mem::MaybeUninit;

// TODO: sort out the names here of types
// TODO: should this not also contain root: Window ?? 
// as ctor ::new has it, yet I take it 
pub struct Drw {
    width: u32,
    height: u32,
    display: &Display,  // why ref to? I imagine drw is made many times and
                        // each refers to the same display? 
    screen: i32,
    drawable: Option<Drawable>, 
    gc: Gc, // ??? maybe just own this raw pointer
    scheme: Option<Clr>, // Assuming scheme should be option as AFAICT
                         // may be null in some functions, e.g. ::text()
                         // But should this be Option<&Clr>?
                         // Believe should own: see set_scheme(). 
                         // As such, Option<Clr> preferred.
    fonts: Option<Fnt>,
}


impl Drw {
    // drawable ctor which appears to be dupl contains root:Window  &
    // window: &Window - is this mistake? If not add to this new() fn
    pub fn new(display: &Display, // see struct def - why ref to? 
               screen:  i32,
               root:    Window,
               width:   u32,
               height:  u32) -> Self {
        todo!();
        Self {
            width: width,
            height: height,
            display: display, 
            screen: screen,
            drawable: Drawable::new(); 
            gc: Gc::new(),
            scheme: None, 
        }

        // This will need to call ecalloc so maybe it needs to 
        // be boxed? Also, XCreatePixmap, XCreateGC, XSetLineAttributes
        // will need to be called. 
    }

    // TODO: figure out what this does. Believe renders text then returns
    // size of text in pixels as i32.
    // This return may also be an error code, but errors return 0 so 
    // doesnt make sense like that....
    // TODO: C here returns 0 if error, e.g. params which might be null
    // cause error. Shouldnt Rust use Result<> instead?
    fn text(&self, x: i32, y: i32, 
        w: u32, h: u32, lpad: u32, text: &str, invert: bool) -> Result<i32, ()> {

        // C: defines a bunch of types here, probably not needed
        let render = x || y || w || h;
        let d: Option<const* XftDraw> = None; // This is option as is only set if !render; TODO:
        // this is a return of a C func so I must manage maybe there is a way to make it easier on
        // myself and not have to manage this 


        // A check for !text exists in C. Does this actually translate
        // to a len check in Rust? TODO
        if self.fonts.is_none() || text.len() == 0 || 
            (render && self.scheme.is_none()) {
                return Err(()); 
        }

        if !render {
            w = ~w; // TODO: is this Rust syntax?
        } else { unsafe {
            XSetForeground(drw.display, drw.gc, 
                drw.scheme[if invert { COLFG } else { COLBG }].pixel);
            XFillRectangle(drw.display, drw.drawable, 
                drw.gc, x, y, w, h);
            d = Some(XftDrawCreate(
                    drw.display, drw.drawable, 
                    DefaultVisual(drw.display, drw.screen), 
                    DefaultColormap(drw.display, drw.screen)
            )
            ); 
            } // end of unsafe

            x += lpad;
            w -= lpad; 
        } // end of else block

        todo!();
        // TODO rest. Some odd looking for loops etc. that might be 
        // difficult to implement. 









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

    // Get width of string provided; internally calls self::text.
    // From C fn: drw_fontset_getwidth
    // BELIEVE_IMPL_COMPLETE
    fn get_width(&self, text: &str) -> u32 {
        if text.len() == 0 || self.fonts.is_none() {
            return 0;
        }

        return self.text(0, 0, 0, 0, 0, text, false);
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
        // Equivalent of drw_clr_create, called only in drw_scm_create
        // We know XftColor will be initialised, as if it is not then 
        // XftColorAllocName will return an error value and the program will
        // die.
        fn create_colour(&self, clrname: &str) -> *mut XftColor {
            if clrname.len() == 0 { return None; }

            let mut dest = MaybeUninit<XftColor>::uninit();

            unsafe { 
                // TODO: add function to get mut ref to these displays
                match XftColorAllocName(
                    self.display
                    DefaultVisual(self.display, self.screen),
                    DefaultColormap(self.display, self.screen),
                    clrname as *[u8], // TODO convert to const char*
                    dest.as_mut_ptr()
                ) as bool {
                    false => { die!("error, cannot allocate color {}", clrname); }
                    true  => dest.assume_init()
                };
            }


        }






        }
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
        self.fonts = Some(set);
    }

    fn set_scheme(&mut self, scm: Clr) { 
        self.scheme = Some(scm);
    }

    // TODO: this function should be largely ok but might still
    // error for something
    fn rect(&self, x: i32, y: i32, 
        w: u32, h: u32, filled: bool, invert: bool) {

        // What to use 
        let ground = if invert { COLBG } else { COLFG };
        
        if self.scheme.is_none() // do I need a check like this?
                // how will I assert that drw->scheme is Some? 
                // will it be assigned at initialisation so always
                // be Some?
        { todo!(); }

        unsafe {
            // TODO: likely need to add .as_raw() to display
            XSetForeground(self.display, self.gc, 
                drw.scheme[ground].pixel);

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



