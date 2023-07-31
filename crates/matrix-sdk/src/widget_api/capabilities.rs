use super::Result;

use super::messages::{self, capabilities::Options, MatrixEvent};

pub type OnEventCallback = Box<dyn Fn(MatrixEvent) + Send>;
/// A wrapper for the matrix client that only exposes what is available through the capabilities.
#[allow(missing_debug_implementations)]
#[derive(Default)]
pub struct Capabilities {
    pub send_room_event: Option<Box<dyn Fn(MatrixEvent) -> Result<()>>>,
    pub add_matrix_room_event_listener: Option<Box<dyn Fn(OnEventCallback)>>,
}
