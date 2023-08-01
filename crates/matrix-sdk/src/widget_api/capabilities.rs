use async_trait::async_trait;

use super::messages::{MatrixEvent, self, capabilities::EventFilter};

/// A wrapper for the matrix client that only exposes what is available through the capabilities.
#[allow(missing_debug_implementations)]
pub struct Capabilities {
    pub send_room_event: Option<Box<dyn Fn(MatrixEvent) + Send + Sync + 'static>>,
    pub event_reader: Option<Box<dyn EventReader>>,
}

#[async_trait]
pub trait EventReader {
    async fn read(&mut self, req: ReadEventRequest) -> Vec<MatrixEvent>;
    fn filter(&self) -> Vec<EventFilter>;
}

#[derive(Debug, Clone)]
pub struct ReadEventRequest {
    pub limit: usize,
    pub kind: EventKind,
}

#[derive(Debug, Clone)]
pub enum EventKind {
    State { key: String },
    Timeline,
}

impl<'t> From<&'t Capabilities> for messages::capabilities::Options {
    fn from(capabilities: &'t Capabilities) -> Self {
        Self {
            send_room_event: capabilities.send_room_event.as_ref().map(|_| vec![]),
            receive_room_event: capabilities.event_reader.as_ref().map(|r| r.filter()),
            receive_state_event: capabilities.event_reader.as_ref().map(|r| r.filter()),
            ..Default::default()
        }
    }
}
