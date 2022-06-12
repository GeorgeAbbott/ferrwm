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

pub fn button_press(event: ButtonPressEvent) {}
pub fn client_message(event: ClientMessageEvent) {}
pub fn configure_request(event: ConfigureRequestEvent) {}
pub fn configure_notify(event: ConfigureNotifyEvent) {}
pub fn destroy_notify(event: DestroyNotifyEvent) {}
pub fn enter_notify(event: EnterNotifyEvent) {}
pub fn expose(event: ExposeEvent) {}
pub fn focus_in(event: FocusInEvent) {}
pub fn key_press(event: KeyPressEvent) {}
pub fn mapping_notify(event: MappingNotifyEvent) {}
pub fn map_request(event: MapRequestEvent) {}
pub fn motion_notify(event: MotionNotifyEvent) {}
pub fn property_notify(event: PropertyNotifyEvent) {}
pub fn unmap_notify(event: UnmapNotifyEvent) {}
