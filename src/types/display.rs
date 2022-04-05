use std::ptr;
use die::die;

// Owns and wraps XDisplay.
pub struct Display {
    inner: const* XDisplay,
}

pub impl Display {
    pub fn new() -> Option<Self> {
        const NULL: i32 = 0;
        unsafe {
            let val = XOpenDisplay(NULL); // const* XDisplay
        }

        // TODO: how do we null check ptrs in Rust?
        if ptr::eq(val, NULL) {
            None
        } else {
            Some(Self {
                inner: val
            });
        }
    }

    // Returns a raw ptr to the inner for use when calling 
    // X functions. Clones the ptr, so caller must enforce that 
    // they will not free etc. inner value, hence marked as 
    // unsafe.
    pub unsafe fn raw_inner(&self) -> *const XDisplay {
        todo!();
    }
}

pub impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.inner);
        }
    }
}




    
