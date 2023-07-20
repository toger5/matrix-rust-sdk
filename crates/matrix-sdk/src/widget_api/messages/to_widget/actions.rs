use serde::{Deserialize, Serialize};

use crate::widget_api::messages::message::ActionBody;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ToWidgetAction {
    #[serde(rename = "capabilities")]
    SendMeCapabilities,
    #[serde(rename = "notify_capabilities")]
    CapabilitiesUpdated,
    #[serde(rename = "notify_capabilities")]
    OpenIdCredentials,
    #[serde(rename = "sent_to_device")]
    SendToDevice(ActionBody<SendToDeviceRequestBody, ()>),
}

#[derive(Serialize, Deserialize, Debug)]
struct SendToDeviceRequestBody {
    #[serde(rename = "type")]
    message_type: String,
    sender: String,
    encrypted: bool,
    messages: serde_json::Value,
}
