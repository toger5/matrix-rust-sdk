// TODO: Remove this supress once we're ready to write the documentation.
#![allow(missing_docs)]

pub mod capabilities;
pub mod driver;
pub mod error;
pub mod handler;
pub mod messages;

use matrix_sdk_base::sync::JoinedRoom;

use self::{driver::{widget_client_driver::ClientFunctions, Driver, widget_matrix_driver::ActualWidgetMatrixDriver}, handler::MessageHandler, messages::Incoming};
pub use self::error::{Error, Result};

pub trait Widget{
    fn id(&self) -> &str;
    fn send(&self, message: &str) -> Result<()>;
    fn get_widget_state_json(&self) -> &str;
}
pub struct WidgetDriver<T: ClientFunctions> {
    handler: MessageHandler<Driver<T>>
}

impl<T: ClientFunctions,W: Widget> WidgetDriver<T> {
    fn new(client_functions: T, widget: W, room: JoinedRoom) -> Self {

        let driver = Driver{
            client_driver: client_functions,
            matrix_driver: ActualWidgetMatrixDriver{room},
            send_to_widget: widget.send,
        };

        WidgetDriver{
            handler: MessageHandler::new(driver)
        }
    }

    fn handle_widget_message(&self, message: &str){
        let widget_message: Incoming = serde_json::from_str(message);
        self.handler.handle(widget_message);
    }

    fn transform_widget_url(url: &str){

    }
}
