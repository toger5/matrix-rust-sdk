use crate::event_handler::EventHandler;
use crate::room::{ Joined, Room};
use crate::Client;

use imbl::HashMap;
use ruma::events::room::message::SyncRoomMessageEvent;
use serde::de::DeserializeOwned;

use super::widget_client_driver::WidgetClientDriver;
use super::widget_matrix_driver::ActualWidgetMatrixDriver;
use super::widget_message::{ WidgetMessage, FromWidgetAction, ToWidgetAction};

pub trait RoomMessageHandler {
    fn handle(ev: SyncRoomMessageEvent, room: Room, client: Client) {}
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub waitForContentLoaded: bool,
}
/// The WidgetDriver is responsible to consume all events emitted by the widget and handle them.
/// It also sends events and responses to the widge.
/// Last it exposes callbacks that the client needs to handle on specific widget requests (for example navigating to a room)
#[derive(Clone, Debug)]
pub struct WidgetClientApi<CD: WidgetClientDriver> {
    widget_id: String,
    // maybe `room` is not even required since this is abstracted into widget_matrix_driver
    room: Option<Joined>,
    widget_settings: Settings,
    // struct that implements all the required client-server matrix functionalities defined in the WidgetMatrixDriver trait (e.g. reading seding matrix events)
    widget_matrix_driver: ActualWidgetMatrixDriver,
    // struct that implements all the required client functionalities defined in the WidgetClientDriver trait (e.g. navigate)
    widget_client_driver: CD,
    // the capabilites need way to get stored to local data.
    capabilities: Vec<String>,
    awaiting_responses: HashMap<String, WidgetMessage>
}

impl<CD: WidgetClientDriver> WidgetClientApi<CD> {
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
        initial_capabilies: Vec<String>,
        widget_settings: Settings,
    ) -> Self {
        let actual_widget_matrix_driver = ActualWidgetMatrixDriver { room: room.clone().unwrap() };

        let driver = WidgetClientApi {
            widget_id: widget_id.to_owned(),
            room: room.clone(),
            widget_matrix_driver: actual_widget_matrix_driver,
            widget_client_driver,
            widget_settings:widget_settings.clone(),
            capabilities: initial_capabilies
        };
        if widget_settings.waitForContentLoaded {
            // driver.to_widget(/*Content Loaded*/);
        }
        driver
    }

    /// # Arguments
    /// * `message` - The raw string of the message received from the widget
    pub async fn from_widget(self, message: &str) {
        // check if reqest/response and direction match
        println!("widget driver handles {}, with room ", message);
        let widget_message: WidgetMessage = serde_json::from_str(message).expect("could not parse msg event");
        self.handle(widget_message);
    }

    pub fn to_widget<Ev, Ctx, H>(&self, handler: H)
    where
        Ev: DeserializeOwned + Send + 'static,
        H: EventHandler<Ev, Ctx>,
    {
        // check if reqest/response and direction match
        // widget_matrix_driver.add_handler(|msg|->handler(msg));
        unimplemented!();
    }
}

impl<CD: WidgetClientDriver> WidgetClientApi<CD> {
    fn handle(&self, message: WidgetMessage) {
        match message {
            WidgetMessage::FromWidget(action) => match action {
                FromWidgetAction::SupportedApiVersions => todo!(),
                FromWidgetAction::ContentLoaded => todo!(),
                FromWidgetAction::GetOpenIDCredentials => todo!(),
                FromWidgetAction::OpenModalWidget => todo!(),
                FromWidgetAction::SendEvent => todo!(),
                FromWidgetAction::SendToDevice(send_to_device_body) => {
                    println!("msg header:{:?}", send_to_device_body);
                    println!("msg apiVersionBody:{:?}", send_to_device_body);
                    // With Daniels code here we should have sth like
                    // self.capabilities.sendToDevice(sendToDeviceBody)
                },
                FromWidgetAction::WatchTurnServers => todo!(),
                FromWidgetAction::UnwatchTurnServers => todo!(),
                FromWidgetAction::MSC2876ReadEvents => todo!(),
                _ => unimplemented!()
            },
            WidgetMessage::ToWidget(action) => match action {
                    ToWidgetAction::SupportedApiVersions(api_version_body) => {
                        println!("msg body:{:?}", api_version_body);
                        println!("msg apiVersionBody:{:?}", api_version_body);
                    },
                    ToWidgetAction::Capabilities(body) => todo!(),
                    ToWidgetAction::NotifyCapabilities => todo!(),
                    ToWidgetAction::OpenIDCredentials(body) => todo!(),
                    ToWidgetAction::WidgetConfig => todo!(),
                    ToWidgetAction::SendEvent => todo!(),
                    ToWidgetAction::SendToDevice(body) => todo!(),
                    ToWidgetAction::UpdateTurnServers => todo!(),
                    _ => unimplemented!()
            },
        }
    }
}

impl<CD: WidgetClientDriver> Drop for WidgetClientApi<CD> {
    fn drop(&mut self) {
        // Add cleanup code here
        println!("Remove event handler");
    }
}
