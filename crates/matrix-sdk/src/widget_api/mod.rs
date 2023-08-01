// TODO: Remove this supress once we're ready to write the documentation.
#![allow(missing_docs)]

pub mod capabilities;
pub mod driver;
pub mod error;
pub mod handler;
pub mod messages;
pub mod widget;

use std::sync::{Arc, Mutex};

pub use self::error::{Error, Result};
use self::{driver::Driver, handler::MessageHandler, widget::Widget};
use crate::room::Joined;

pub struct WidgetDriver<W: Widget> {
    pub handler: Arc<Mutex<MessageHandler<Driver<W>>>>,
}

impl<W: Widget + 'static> WidgetDriver<W> {
    fn new(widget: W, room: Joined) -> Self {
        let driver = Driver { widget, matrix_room: room, add_event_handler_handle: None };
        let handler = Arc::new(Mutex::new(MessageHandler::new(driver)));
        let widget_driver = WidgetDriver { handler: handler.clone() };

        widget_driver.transport.on_incoming(Box::new(move |req| {
            handler.lock().unwrap().handle(req);
        }));

        widget_driver
    }

    fn handle_widget_message(&self, message: &str) {
        self.handler.handle(message);
    }

    fn transform_widget_url(&self, url: &str) {}
}
