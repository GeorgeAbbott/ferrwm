#![allow(unused_variables)]

use log::{trace, debug, warn, error, info};
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
    
    
        debug!("Keypress value: {}", keypress);
        logf(format!("keypress value: {}", keypress).as_str());
    }
    pub fn mapping_notify(&self, event: MappingNotifyEvent) {}
    pub fn map_request(&self, event: MapRequestEvent) {}
    pub fn motion_notify(&self, event: MotionNotifyEvent) {}
    pub fn property_notify(&self, event: PropertyNotifyEvent) {}
    pub fn unmap_notify(&self, event: UnmapNotifyEvent) {}
}
