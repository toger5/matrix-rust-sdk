use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

use super::{
    messages::{
        capabilities::{Filter, Options},
        from_widget::{
            ReadEventRequest, ReadEventResponse, SendEventRequest, SendEventResponse,
            SendToDeviceRequest,
        },
        MatrixEvent,
    },
    Result,
};

/// A wrapper for the matrix client that only exposes what is available through the capabilities.
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct Capabilities {
    // Room and state events use the same sender, reader, listener
    // on the rust-sdk side room and state events dont make a difference for the transport.
    // It is the widgets responsibility to differenciate and react to them accordingly.
    pub event_listener: Option<UnboundedReceiver<MatrixEvent>>,

    pub event_reader: Option<Box<dyn EventReader>>,
    pub event_sender: Option<Box<dyn EventSender>>,
    // TODO implement to_device_sender (not required for EC->EX)
    // pub to_device_sender: Option<Box<dyn ToDeviceSender>>,
}

#[async_trait]
pub trait EventReader {
    async fn read(&self, req: ReadEventRequest) -> Result<ReadEventResponse>;
    fn get_filter(&self) -> &Vec<Filter>;
}

#[async_trait]
pub trait EventSender {
    async fn send(&self, req: SendEventRequest) -> Result<SendEventResponse>;
    fn get_filter(&self) -> &Vec<Filter>;
}

#[async_trait]
pub trait ToDeviceSender {
    async fn send(&self, req: SendToDeviceRequest) -> Result<()>;
}

impl<'t> From<&'t Capabilities> for Options {
    fn from(capabilities: &'t Capabilities) -> Self {
        Options {
            // room events
            read_filter: capabilities
                .event_reader
                .as_ref()
                .map(|r| r.get_filter().clone())
                .unwrap_or_default(),
            send_filter: capabilities
                .event_sender
                .as_ref()
                .map(|r| r.get_filter().clone())
                .unwrap_or_default(),

            // all other unimplemented capabilities
            ..Options::default()
        }
    }
}
