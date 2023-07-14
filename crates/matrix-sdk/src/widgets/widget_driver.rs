use crate::event_handler::EventHandler;
use crate::room::{ Joined, Room};
use crate::Client;

use ruma::api::client::discovery::get_capabilities;
use ruma::events::room::message::SyncRoomMessageEvent;
use serde::de::DeserializeOwned;

use super::widget_client_driver::WidgetClientDriver;
use super::widget_matrix_driver::{ActualWidgetMatrixDriver, WidgetMatrixDriver};
use super::widget_message::{WidgetAction, WidgetMessage, FromWidgetAction, ToWidgetAction};

pub trait RoomMessageHandler {
    fn handle(ev: SyncRoomMessageEvent, room: Room, client: Client) {}
}
#[derive(Clone)]

struct Settings {
    waitForContentLoaded: bool,
}
/// The WidgetDriver is responsible to consume all events emitted by the widget and handle them.
/// It also sends events and responses to the widge.
/// Last it exposes callbacks that the client needs to handle on specific widget requests (for example navigating to a room)
#[derive(Clone)]
pub struct WidgetDriver<CD: WidgetClientDriver> {
    widget_id: String,
    // maybe `room` is not even required since this is abstracted into widget_matrix_driver
    room: Option<Joined>,
    widget_settings: Settings,
    // struct that implements all the required client-server matrix functionalities defined in the WidgetMatrixDriver trait (e.g. reading seding matrix events)
    widget_matrix_driver: ActualWidgetMatrixDriver,
    // struct that implements all the required client functionalities defined in the WidgetClientDriver trait (e.g. navigate)
    widget_client_driver: CD,
    // the capabilites need way to get stored to local data.
    capabilities: Capabilities,
}

impl<CD: WidgetClientDriver> WidgetDriver<CD> {
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
        initial_capabilies: Capabilities
        // widget_matrix_driver: Option<MD>,
    ) -> Self {
        let actual_widget_matrix_driver = ActualWidgetMatrixDriver { room: room.clone().unwrap() };
        // if let Some(widget_md) = widget_matrix_driver {
        //     actual_widget_matrix_driver = widget_md;
        // }
        let driver = WidgetDriver {
            widget_id: widget_id.to_owned(),
            room: room.clone(),
            widget_matrix_driver: actual_widget_matrix_driver,
            widget_client_driver,
        };
        if(widget_settings.waitForContentLoaded){
            self.to_widget(/*Content Loaded*/);
        }
        driver
    }

    /// # Arguments
    /// * `message` - The raw string of the message received from the widget
    pub async fn from_widget(self, message: &str) {
        // check if reqest/response and direction match
        println!("widget driver handles {}, with room ", message);
        self.handle(message);
    }

    pub fn to_widget<Ev, Ctx, H>(&self, handler: H)
    where
        Ev: DeserializeOwned + Send + 'static,
        H: EventHandler<Ev, Ctx>,
    {
        // check if reqest/response and direction match
        widget_matrix_driver.add_handler(|msg|->handler(msg));
        unimplemented!();
    }

    pub fn get_capabilities() -> Capabilities{
        unimplemented!()
    }
}

impl<CD: WidgetClientDriver> WidgetDriver<CD> {
    fn handle(&self, message: &str) {
        let msg: WidgetMessage = serde_json::from_str(message).expect("could not parse msg event");
        let WidgetMessage::Request(msg_req) = msg;
        match msg_req.action {
            WidgetAction::FromWidget(action) => match action {
                FromWidgetAction::CloseModalWidget => unimplemented!(),
                FromWidgetAction::SupportedApiVersions => todo!(),
                FromWidgetAction::ContentLoaded => todo!(),
                FromWidgetAction::SendSticker => unimplemented!(),
                FromWidgetAction::UpdateAlwaysOnScreen => unimplemented!(),
                FromWidgetAction::GetOpenIDCredentials => todo!(),
                FromWidgetAction::OpenModalWidget => todo!(),
                FromWidgetAction::SetModalButtonEnabled => unimplemented!(),
                FromWidgetAction::SendEvent => todo!(),
                FromWidgetAction::SendToDevice => todo!(),
                FromWidgetAction::WatchTurnServers => todo!(),
                FromWidgetAction::UnwatchTurnServers => todo!(),
                FromWidgetAction::MSC2876ReadEvents => todo!(),
                FromWidgetAction::MSC2931Navigate => unimplemented!(),
                FromWidgetAction::MSC2974RenegotiateCapabilities => unimplemented!(),
                FromWidgetAction::MSC3869ReadRelations => unimplemented!(),
                FromWidgetAction::MSC3973UserDirectorySearch => unimplemented!(),
            },
                WidgetAction::ToWidget(action) => match action {
                    ToWidgetAction::SupportedApiVersions => todo!(),
                    ToWidgetAction::Capabilities => todo!(),
                    ToWidgetAction::NotifyCapabilities => todo!(),
                    ToWidgetAction::TakeScreenshot => unimplemented!(),
                    ToWidgetAction::UpdateVisibility => unimplemented!(),
                    ToWidgetAction::OpenIDCredentials => todo!(),
                    ToWidgetAction::WidgetConfig => todo!(),
                    ToWidgetAction::CloseModalWidget => unimplemented!(),
                    ToWidgetAction::ButtonClicked => unimplemented!(),
                    ToWidgetAction::SendEvent => todo!(),
                    ToWidgetAction::SendToDevice => todo!(),
                    ToWidgetAction::UpdateTurnServers => todo!(),
            },
        }
    }
}

impl<CD: WidgetClientDriver> Drop for WidgetDriver<CD> {
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
