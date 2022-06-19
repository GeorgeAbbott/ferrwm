#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use log::{trace, debug, warn, error, info};
use x11rb::protocol::xproto::KeyButMask;
use crate::{utils::logf, wm::WindowManager};
#[allow(unused)]
// Holds functions to handle events which are called from handle_event
// function in main.rs.

use x11rb::protocol::{
    xproto::{
        ButtonPressEvent, ClientMessageEvent, ConfigureNotifyEvent, ConfigureRequestEvent,
        DestroyNotifyEvent, EnterNotifyEvent, ExposeEvent, FocusInEvent, KeyPressEvent,
        MapRequestEvent, MappingNotifyEvent, MotionNotifyEvent, PropertyNotifyEvent,
        UnmapNotifyEvent,
    },
    Event,
};
impl<'wm, 'rc> WindowManager<'wm, 'rc> {
    pub fn button_press(&self, event: ButtonPressEvent) {}
    pub fn client_message(&self, event: ClientMessageEvent) {}
    pub fn configure_request(&self, event: ConfigureRequestEvent) {}
    pub fn configure_notify(&self, event: ConfigureNotifyEvent) {}
    pub fn destroy_notify(&self, event: DestroyNotifyEvent) {}
    pub fn enter_notify(&self, event: EnterNotifyEvent) {}
    pub fn expose(&self, event: ExposeEvent) {}
    pub fn focus_in(&self, event: FocusInEvent) {}
    pub fn key_press(&self, event: KeyPressEvent) {
        trace!("Entered key_press");
        logf("Entered key_press");
        
        let keypress = event.detail;
        let keystate = event.state; // for my reference, this is a mask of 
                                // mods at the time. It's over at 
                                // xcb.freedesktop.org/tutorial/events
                                // some halfway down

        // written like this so the warnings will stop hassling me, think this 
        // is still correct
        let ctrl_w = keypress == 25 && (keystate & u16::from(KeyButMask::SHIFT)) != 0;

    
        debug!("Keypress value: {}", keypress);
        logf(format!("keypress value: {}", keypress).as_str());
    }
    pub fn mapping_notify(&self, event: MappingNotifyEvent) {}
    pub fn map_request(&self, event: MapRequestEvent) {}
    pub fn motion_notify(&self, event: MotionNotifyEvent) {}
    pub fn property_notify(&self, event: PropertyNotifyEvent) {}
    pub fn unmap_notify(&self, event: UnmapNotifyEvent) {}
}
