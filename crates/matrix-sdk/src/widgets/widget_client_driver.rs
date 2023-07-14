use ruma::api::client::discovery::get_capabilities::Capabilities;

use crate::room::Joined;

pub trait WidgetClientDriver {
    /// Navigates the client with a matrix.to URI. In future this function will also be provided
    /// with the Matrix URIs once matrix.to is replaced. The given URI will have already been
    /// lightly checked to ensure it looks like a valid URI, though the implementation is recommended
    /// to do further checks on the URI.
    /// # Arguments
    /// * `uri` - The URI to navigate to.
    fn navigate(uri: &str);

    fn show_capability_request(cap: Capabilities);
}

struct ActualWidgetClientDriver {
    room: Joined,
}

impl WidgetClientDriver for ActualWidgetClientDriver {
    fn navigate(uri: &str) {
        unimplemented!()
    }
}
