use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use url::Url;

use crate::widget_api::messages::message::{Message, MessageBody};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum FromWidgetAction {
    #[serde(rename = "supported_api_versions")]
    GetSupportedApiVersion(MessageBody<(), SupportedVersions>),
    #[serde(rename = "content_loaded")]
    ContentLoaded(MessageBody<(), ()>),
    #[serde(rename = "org.matrix.msc2931.navigate")]
    Navigate(MessageBody<Url, Result<(), &'static str>>),
    GetOpenId(MessageBody<(), GetOpenIdResponse>),
    SendToDevice(MessageBody<SendToDeviceRequest, ()>)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportedVersions {
    pub versions: Vec<ApiVersion>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiVersion {
    PreRelease,
}

// MSC1960

#[derive(Serialize, Deserialize, Debug)]
pub enum OpenIdState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "request")]
    PendingUserConfirmation,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetOpenIdResponse {
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
    message_type: String,
    encrypted: bool,
    content: HashMap<String, HashMap<String, serde_json::Value>>,
}