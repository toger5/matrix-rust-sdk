use std::sync::{Arc, RwLock};

use crate::room::{Common, Joined, Room};
use crate::{BaseRoom, Client, Result, RoomState};
use backoff::default;
use futures_core::Future;
use http::request;
use ruma::events::room::message::{
    OriginalSyncRoomMessageEvent, RoomMessageEventContent, SyncRoomMessageEvent,
};
use ruma::events::SyncMessageLikeEvent;

use crate::widgets::widget_request::{WidgetApiRequest, WidgetApiDirection, WidgetApiFromWidgetAction, WidgetApiToWidgetAction};
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
        let request = WidgetApiRequest{
            api: WidgetApiDirection::FromWidget,
            request_id: String::from("request_id1234"),
            action: WidgetApiFromWidgetAction::ContentLoaded,
            widget_id: String::from("widget_id1234"),
            data: serde_json::json!({"data":"10"}),
        }
        // here we want to have a big match
        match request.action {
            WidgetApiFromWidgetAction::ContentLoaded => self.handle_content_loaded(request),
            WidgetApiFromWidgetAction::MSC2876ReadEvents => self.handle_read_events(request),
            default => self.handle_unimplemented_request(request)
        }
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

// implement handle function (might should get split into its own file)
impl WidgetDriver{
    fn handle_content_loaded(&self, request: WidgetApiRequest){

    }
    fn handle_read_events(&self, request: WidgetApiRequest){

    }
    fn handle_unimplemented_request(&self, request: WidgetApiRequest){
        let delegate = Arc::clone(&self.toWidgetDelegate);
        if let Some(delegate) = delegate.read().unwrap().as_ref()
                    {
                        delegate.to_widget(request.)
                    }
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
    fn to_widget(&self, request: serde_json::Value);
}

#[uniffi::export(callback_interface)]
pub trait CapabilityDelegate: Sync + Send {
    fn did_receive_capability_request(&self);
}
