use crate::event_handler::EventHandler;
use crate::room::{ Joined, Room};
use crate::Client;

use ruma::api::client::discovery::get_capabilities;
use ruma::events::room::message::SyncRoomMessageEvent;
use serde::de::DeserializeOwned;

use super::widget_client_driver::WidgetClientDriver;
use super::widget_matrix_driver::{ActualWidgetMatrixDriver, WidgetMatrixDriver};
use super::widget_message::{WidgetActionBody, WidgetMessage, FromWidgetActionBody, ToWidgetActionBody};

pub trait RoomMessageHandler {
    fn handle(ev: SyncRoomMessageEvent, room: Room, client: Client) {}
}
#[derive(Clone)]

pub struct Settings {
    pub waitForContentLoaded: bool,
}
/// The WidgetDriver is responsible to consume all events emitted by the widget and handle them.
/// It also sends events and responses to the widge.
/// Last it exposes callbacks that the client needs to handle on specific widget requests (for example navigating to a room)
#[derive(Clone)]
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
    capabilities: Vec<Capability>,
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
        initial_capabilies: Vec<Capability>,
        widget_settings: Settings,
        // widget_matrix_driver: Option<MD>,
    ) -> Self {
        let actual_widget_matrix_driver = ActualWidgetMatrixDriver { room: room.clone().unwrap() };
        // if let Some(widget_md) = widget_matrix_driver {
        //     actual_widget_matrix_driver = widget_md;
        // }
        let driver = WidgetClientApi {
            widget_id: widget_id.to_owned(),
            room: room.clone(),
            widget_matrix_driver: actual_widget_matrix_driver,
            widget_client_driver,
            widget_settings:widget_settings.clone(),
            capabilities: initial_capabilies
        };
        if(widget_settings.waitForContentLoaded){
            // driver.to_widget(/*Content Loaded*/);
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
        // widget_matrix_driver.add_handler(|msg|->handler(msg));
        unimplemented!();
    }

    pub fn get_capabilities() -> Vec<Capability>{
        unimplemented!()
    }
}

impl<CD: WidgetClientDriver> WidgetClientApi<CD> {
    fn handle(&self, message: &str) {
        let msg: WidgetMessage = serde_json::from_str(message).expect("could not parse msg event");
        match msg {
            WidgetMessage::FromWidget(msgContent) => match msgContent.action {
                FromWidgetActionBody::CloseModalWidget => unimplemented!(),
                FromWidgetActionBody::SupportedApiVersions => todo!(),
                FromWidgetActionBody::ContentLoaded => todo!(),
                FromWidgetActionBody::SendSticker => unimplemented!(),
                FromWidgetActionBody::UpdateAlwaysOnScreen => unimplemented!(),
                FromWidgetActionBody::GetOpenIDCredentials => todo!(),
                FromWidgetActionBody::OpenModalWidget => todo!(),
                FromWidgetActionBody::SetModalButtonEnabled => unimplemented!(),
                FromWidgetActionBody::SendEvent => todo!(),
                FromWidgetActionBody::SendToDevice => todo!(),
                FromWidgetActionBody::WatchTurnServers => todo!(),
                FromWidgetActionBody::UnwatchTurnServers => todo!(),
                FromWidgetActionBody::MSC2876ReadEvents => todo!(),
                FromWidgetActionBody::MSC2931Navigate => unimplemented!(),
                FromWidgetActionBody::MSC2974RenegotiateCapabilities => unimplemented!(),
                FromWidgetActionBody::MSC3869ReadRelations => unimplemented!(),
                FromWidgetActionBody::MSC3973UserDirectorySearch => unimplemented!(),
            },
            WidgetMessage::ToWidget(msgContent) => match msgContent.action {
                    ToWidgetActionBody::SupportedApiVersions(apiVersionBody) => {
                        println!("msg header:{:?}", msgContent.header);
                        println!("msg apiVersionBody:{:?}", apiVersionBody);
                        // todo!()
                    },
                    ToWidgetActionBody::Capabilities(action) => todo!(),
                    ToWidgetActionBody::NotifyCapabilities => todo!(),
                    ToWidgetActionBody::TakeScreenshot => unimplemented!(),
                    ToWidgetActionBody::UpdateVisibility => unimplemented!(),
                    ToWidgetActionBody::OpenIDCredentials(action) => todo!(),
                    ToWidgetActionBody::WidgetConfig => todo!(),
                    ToWidgetActionBody::CloseModalWidget => unimplemented!(),
                    ToWidgetActionBody::ButtonClicked => unimplemented!(),
                    ToWidgetActionBody::SendEvent => todo!(),
                    ToWidgetActionBody::SendToDevice => todo!(),
                    ToWidgetActionBody::UpdateTurnServers => todo!(),
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

#[derive(Clone)]
pub enum Capability {
    ReadMessages,
    SendMessages,
}
