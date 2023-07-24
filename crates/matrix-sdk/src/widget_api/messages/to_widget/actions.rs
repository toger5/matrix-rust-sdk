use serde::{Deserialize, Serialize};

use crate::widget_api::messages::{message::ActionBody, OpenIdState, MatrixEvent};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ToWidgetAction {
    #[serde(rename = "capabilities")]
    SendMeCapabilities(ActionBody<(), SendMeCapabilitiesResponse>),
    #[serde(rename = "notify_capabilities")]
    CapabilitiesUpdated(ActionBody<CapabilitiesUpdatedRequest, ()>),
    #[serde(rename = "openid_credentials")]
    OpenIdCredentials(ActionBody<OpenIdCredentialsRequest, ()>),
    #[serde(rename = "sent_to_device")]
    SendToDevice(ActionBody<SendToDeviceRequest, ()>),
    #[serde(rename = "send_event")]
    SendEvent(ActionBody<MatrixEvent, ()>)
}

#[derive(Serialize, Deserialize, Debug)]
struct SendMeCapabilitiesResponse {
    capabilities: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenIdCredentialsRequest {
    state: OpenIdState, //OpenIDRequestState;
    original_request_id: String,
    access_token: Option<String>,
    expires_in: Option<i32>,
    matrix_server_name: Option<String>,
    token_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SendToDeviceRequest {
    #[serde(rename = "type")]
    event_type: String,
    sender: String,
    encrypted: bool,
    messages: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug)]
struct CapabilitiesUpdatedRequest {
    requested: Vec<String>,
    approved: Vec<String>
}

