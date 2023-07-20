use ruma::api::client::discovery::get_capabilities::Capabilities;

pub trait WidgetClientDriver {
    /// This is the most important message to be implemented in the client driver
    /// the function will be called on the struct implementing the client driver whenever there is a new message available
    /// that has to be sent to the widget
    fn send_widget_message(message: &str);
    /// Navigates the client with a matrix.to URI. In future this function will also be provided
    /// with the Matrix URIs once matrix.to is replaced. The given URI will have already been
    /// lightly checked to ensure it looks like a valid URI, though the implementation is recommended
    /// to do further checks on the URI.
    /// # Arguments
    /// * `uri` - The URI to navigate to.
    fn navigate(uri: &str);

    fn show_capability_request(cap: Capabilities);
}

