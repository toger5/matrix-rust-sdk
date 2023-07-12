use std::sync::{Arc, RwLock};

use crate::room::{Common, Joined, Room};
use crate::{BaseRoom, Client, Result, RoomState};
use futures_core::Future;
use ruma::events::room::message::{
    OriginalSyncRoomMessageEvent, RoomMessageEventContent, SyncRoomMessageEvent,
};
use ruma::events::SyncMessageLikeEvent;
#[derive(Clone)]
pub struct WidgetDriver {
    room: Option<Joined>,
    toWidgetDelegate: Arc<RwLock<Option<Box<dyn ToWidgetDelegate>>>>,
}

impl WidgetDriver {
    // What should we use to emit events that should be send to the widget?
    // - are there events in the FFI? -
    // - are can the widgetDriver be initialized with callbacks?
    /// # Arguments
    /// * `room` - The underlying room.
    pub fn new(room: Option<Joined>) -> Self {
        let driver =
            WidgetDriver { room: room.clone(), toWidgetDelegate: Arc::new(RwLock::new(None)) };
        let driver_room = driver.room.clone().unwrap();
        let handler = |ev: SyncRoomMessageEvent, room: Room, client: Client| async move {
            // Common usage: Room event plus room and client.
            println!("WidgetDriver handle event: {:?}", ev)
        };
        driver_room.inner.client.add_event_handler(handler);
        driver
    }
    async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
        println!("WidgetDriver handle event")
    }

    /// # Arguments
    /// * `json` - The widget event json.
    pub async fn from_widget(self, json: &str) {
        println!("widget driver handles {}, with room ", json);
        let content =
            RoomMessageEventContent::text_plain(json.to_owned() + &String::from("normal send"));
        let r = self.room.clone().unwrap();
        let _ = r.send_raw(serde_json::json!({"body":"test"}), "customWidgetType", None).await;
        let _ = r.send(content, None).await;
    }

    pub fn set_to_widget_delegate(&self, delegate: Option<Box<dyn ToWidgetDelegate>>) {
        *self.toWidgetDelegate.write().unwrap() = delegate;
    }
}

impl Drop for WidgetDriver {
    fn drop(&mut self) {
        // Add cleanup code here
        println!("Remove event handler");
    }
}
#[derive(Debug, Clone)]
struct Capabilities {
    send_messages: String,
}

#[uniffi::export(callback_interface)]
pub trait ToWidgetDelegate: Sync + Send {
    fn to_widget(&self, is_soft_logout: bool);
}

#[uniffi::export(callback_interface)]
pub trait CapabilityDelegate: Sync + Send {
    fn did_receive_capability_request(&self);
}
