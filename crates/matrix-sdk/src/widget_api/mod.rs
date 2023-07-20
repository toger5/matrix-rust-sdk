// TODO: Remove this supress once we're ready to write the documentation.
#![allow(missing_docs)]

pub mod capabilities;
pub mod driver;
pub mod error;
pub mod handler;
pub mod messages;

use matrix_sdk_base::sync::JoinedRoom;

use self::{driver::{widget_client_driver::WidgetClientDriver, Driver, widget_matrix_driver::ActualWidgetMatrixDriver}, handler::MessageHandler, messages::Incoming};
pub use self::error::{Error, Result};

pub struct WidgetDriver<T: WidgetClientDriver> {
    handler: MessageHandler<Driver<T>>
}

impl<T: WidgetClientDriver> WidgetDriver<T> {
    fn new(client_delegate: T, room: JoinedRoom) -> Self {

        let driver = Driver{
            client_driver: client_delegate,
            matrix_driver: ActualWidgetMatrixDriver{room},
            send_to_widget: client_delegate.send_widget_message,
        };

        WidgetDriver{
            handler: MessageHandler::new(driver)
        }
    }

    fn handle_widget_message(&self, message: &str){
        let widget_message: Incoming = serde_json::from_str(message);
        self.handler.handle(widget_message);
    }
}
