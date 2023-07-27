// TODO: Remove this supress once we're ready to write the documentation.
#![allow(missing_docs)]

pub mod capabilities;
pub mod driver;
pub mod error;
pub mod handler;
pub mod messages;
pub mod trasport;
pub mod widget;

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
    handler: MessageHandler<Driver<W>>,
}

impl<W: Widget> WidgetDriver<W> {
    fn new(widget: W, room: Joined) -> Self {
        let driver = Driver { widget, matrix_driver: RustSdkMatrixDriver { room } };
        let mut handler = MessageHandler::new(driver);
        let widget_driver = WidgetDriver { transport: DummyTransport {}, handler };

        widget_driver.transport.on_incoming(Box::new(|req| {
            handler.handle(req);
        }));
        
        widget_driver
    }

    fn handle_widget_message(&self, message: &str) {
        self.transport.receive(message);
    }

    fn transform_widget_url(url: &str) {}
}
