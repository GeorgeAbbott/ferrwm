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

fn button_press(event: ButtonPressEvent) {}
fn client_message(event: ClientMessageEvent) {}
fn configure_request(event: ConfigureRequestEvent) {}
fn configure_notify(event: ConfigureNotifyEvent) {}
fn destroy_notify(event: DestroyNotifyEvent) {}
fn enter_notify(event: EnterNotifyEvent) {}
fn expose(event: ExposeEvent) {}
fn focus_in(event: FocusInEvent) {}
fn key_press(event: KeyPressEvent) {}
fn mapping_notify(event: MappingNotifyEvent) {}
fn map_request(event: MapRequestEvent) {}
fn motion_notify(event: MotionNotifyEvent) {}
fn property_notify(event: PropertyNotifyEvent) {}
fn unmap_notify(event: UnmapNotifyEvent) {}
