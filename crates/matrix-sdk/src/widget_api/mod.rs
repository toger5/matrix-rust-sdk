// TODO: Remove this supress once we're ready to write the documentation.
#![allow(missing_docs)]

pub mod capabilities;
pub mod driver;
pub mod error;
pub mod handler;
pub mod messages;
pub mod trasport;
pub mod widget;

use std::sync::{Arc, Mutex};

pub use self::error::{Error, Result};
use self::{
    driver::{Driver, RustSdkMatrixDriver},
    handler::MessageHandler,
    trasport::{DummyTransport, Transport},
    widget::Widget,
};
use crate::room::Joined;

pub struct WidgetDriver<W: Widget> {
    transport: DummyTransport,
    pub handler: Arc<Mutex<MessageHandler<Driver<W>>>>,
}

impl<W: Widget + 'static> WidgetDriver<W> {
    fn new(widget: W, room: Joined) -> Self {
        let driver = Driver { widget, matrix_driver: RustSdkMatrixDriver { room } };
        let handler = Arc::new(Mutex::new(MessageHandler::new(driver)));
        let widget_driver = WidgetDriver { transport: DummyTransport {}, handler: handler.clone() };

        widget_driver.transport.on_incoming(Box::new(move |req| {
            handler.lock().unwrap().handle(req);
        }));

        widget_driver
    }

    fn handle_widget_message(&self, message: &str) {
        self.transport.receive(message);
    }

    fn transform_widget_url(url: &str) {}
}
