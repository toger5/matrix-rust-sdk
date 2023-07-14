use std::sync::{Arc, RwLock};

use crate::event_handler::EventHandler;
use crate::room::{Common, Joined, Room};
use crate::{BaseRoom, Client, Result, RoomState};
use backoff::default;
use futures_core::Future;
use http::request;
use ruma::events::room::message::{
    OriginalSyncRoomMessageEvent, RoomMessageEventContent, SyncRoomMessageEvent,
};
use ruma::events::SyncMessageLikeEvent;
use serde::de::DeserializeOwned;

use crate::widgets::{
    widget_handler::WidgetMessageHandler,
    widget_message::{
        FromWidgetAction, ToWidgetAction, WidgetMessageDirection, WidgetMessageRequest,
    },
};

use super::widget_client_driver::{self, WidgetClientDriver};
use super::widget_matrix_driver::{self, ActualMatrixClientDriver, WidgetMatrixDriver};
use super::widget_message::{WidgetAction, WidgetMessage};

pub trait RoomMessageHandler {
    fn handle(ev: SyncRoomMessageEvent, room: Room, client: Client) {}
}

/// The WidgetDriver is responsible to consume all events emitted by the widget and handle them.
/// It also sends events and responses to the widge.
/// Last it exposes callbacks that the client needs to handle on specific widget requests (for example navigating to a room)
#[derive(Clone)]
pub struct WidgetDriver<CD: WidgetClientDriver, MD: WidgetMatrixDriver> {
    widget_id: String,
    // maybe `room` is not even required since this is abstracted into widget_matrix_driver
    room: Option<Joined>,
    // struct that implements all the required client-server matrix functionalities defined in the WidgetMatrixDriver trait (e.g. reading seding matrix events)
    widget_matrix_driver: MD,
    // struct that implements all the required client functionalities defined in the WidgetClientDriver trait (e.g. navigate)
    widget_client_driver: CD,
}

impl<CD, MD> WidgetDriver<CD, MD> {
    // What should we use to emit events that should be send to the widget?
    // - are there events in the FFI? -
    // - are can the widgetDriver be initialized with callbacks?
    /// # Arguments
    /// * `room` - The underlying room.
    /// * `widget_client_driver` - A struct implementing all the client specific widget functionalities (e.g. navigate to a room)
    /// * `widget_matrix_driver` - A struct implementing all the matrix specific widget functionalities (eg. read/send events, sending to device messages, getting oidc tokens ...)
    pub fn new(
        widget_id: &str,
        room: Option<Joined>,
        widget_client_driver: CD,
        widget_matrix_driver: Option<MD>,
    ) -> Self {
        let actual_widget_matrix_driver = ActualMatrixClientDriver { room: room.clone() };
        if let Some(widget_md) = widget_matrix_driver {
            actual_widget_matrix_driver = widget_md;
        }
        let driver = WidgetDriver {
            widget_id: widget_id.to_owned(),
            room: room.clone(),
            widget_matrix_driver: actual_widget_matrix_driver,
            widget_client_driver: widget_client_driver,
        };
        driver
    }

    /// # Arguments
    /// * `message` - The raw string of the message received from the widget
    pub async fn from_widget(self, message: &str) {
        println!("widget driver handles {}, with room ", message);
        self.handle(message);
    }

    pub fn to_widget<Ev, Ctx, H>(&self, handler: H)
    where
        Ev: DeserializeOwned + Send + 'static,
        H: EventHandler<Ev, Ctx>,
    {
        unimplemented!()
    }
}

impl WidgetDriver {
    fn handle(message: &str) {
        let msg: WidgetMessage = serde_json::from_value(serde_json::json!(message));
        let WidgetMessage::Request(msg_req) = msg;
        match msg_req.action {
            WidgetAction::FromWidget(action) => match action {
                FromWidgetAction::CloseModalWidget => todo!(),
                FromWidgetAction::SupportedApiVersions => todo!(),
                FromWidgetAction::ContentLoaded => todo!(),
                FromWidgetAction::SendSticker => todo!(),
                FromWidgetAction::UpdateAlwaysOnScreen => todo!(),
                FromWidgetAction::GetOpenIDCredentials => todo!(),
                FromWidgetAction::OpenModalWidget => todo!(),
                FromWidgetAction::SetModalButtonEnabled => todo!(),
                FromWidgetAction::SendEvent => todo!(),
                FromWidgetAction::SendToDevice => todo!(),
                FromWidgetAction::WatchTurnServers => todo!(),
                FromWidgetAction::UnwatchTurnServers => todo!(),
                FromWidgetAction::MSC2876ReadEvents => todo!(),
                FromWidgetAction::MSC2931Navigate => todo!(),
                FromWidgetAction::MSC2974RenegotiateCapabilities => todo!(),
                FromWidgetAction::MSC3869ReadRelations => todo!(),
                FromWidgetAction::MSC3973UserDirectorySearch => todo!(),
            },
            WidgetAction::ToWidget(action) => match action {
                ToWidgetAction::SupportedApiVersions => todo!(),
                ToWidgetAction::Capabilities => todo!(),
                ToWidgetAction::NotifyCapabilities => todo!(),
                ToWidgetAction::TakeScreenshot => todo!(),
                ToWidgetAction::UpdateVisibility => todo!(),
                ToWidgetAction::OpenIDCredentials => todo!(),
                ToWidgetAction::WidgetConfig => todo!(),
                ToWidgetAction::CloseModalWidget => todo!(),
                ToWidgetAction::ButtonClicked => todo!(),
                ToWidgetAction::SendEvent => todo!(),
                ToWidgetAction::SendToDevice => todo!(),
                ToWidgetAction::UpdateTurnServers => todo!(),
            },
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
pub enum Capability {
    ReadMessages,
    SendMessages,
}
