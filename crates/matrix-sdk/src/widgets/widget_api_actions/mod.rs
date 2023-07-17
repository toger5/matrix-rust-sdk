use serde::{Serialize, Deserialize};

pub mod capabilities;
pub mod open_id_credentials;
pub mod supported_api_versions;
pub mod send_to_device;

#[derive(Serialize, Deserialize, Debug)]
pub struct WidgetMessageEmptyData {}


#[derive(Serialize, Deserialize, Debug)]
pub struct WidgetError {
    error: WidgetErrorMessage
}
impl WidgetError{
    pub fn new(message: &str) -> Self {
        WidgetError { error: WidgetErrorMessage { message: message.to_owned() } }
    }
    pub fn message(&self) -> String {
        self.error.message.clone()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WidgetErrorMessage {
    message: String
}