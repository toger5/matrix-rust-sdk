use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use url::Url;

use crate::widget_api::messages::{
    message::MessageBody, MatrixEvent, OpenIdState, SupportedVersions, ReadRelationsDirection,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum FromWidgetAction {
    #[serde(rename = "supported_api_versions")]
    GetSupportedApiVersion(MessageBody<(), SupportedVersions>),
    #[serde(rename = "content_loaded")]
    ContentLoaded(MessageBody<(), ()>),
    #[serde(rename = "org.matrix.msc2931.navigate")]
    Navigate(MessageBody<Url, Result<(), &'static str>>),
    #[serde(rename = "get_openid")]
    GetOpenId(MessageBody<(), GetOpenIdResponse>),
    #[serde(rename = "send_to_device")]
    SendToDevice(MessageBody<SendToDeviceRequest, ()>),
    #[serde(rename = "send_events")]
    SendEvent(MessageBody<SendEventRequest, SendEventResponse>),
    #[serde(rename = "org.matrix.msc2876.read_events")]
    ReadEvent(MessageBody<ReadEventRequest, ReadEventResponse>),
    #[serde(rename = "org.matrix.msc3869.read_relations")]
    ReadRelations(MessageBody<ReadEventRequest, ReadEventResponse>),
}

// MSC1960

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOpenIdResponse {
    state: OpenIdState, //OpenIDRequestState;
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

#[derive(Serialize, Deserialize, Debug)]
struct SendEventRequest {
    #[serde(rename = "type")]
    message_type: String,
    state_key: String,
    content: serde_json::Value,
}
#[derive(Serialize, Deserialize, Debug)]
struct SendEventResponse {
    room_id: String,
    event_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReadEventRequest {
    #[serde(rename = "type")]
    message_type: String,
    state_key: String,
    limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReadEventResponse {
    events: Vec<MatrixEvent>,
}

// MSC3869
#[derive(Serialize, Deserialize, Debug)]
struct ReadRelationsRequest {
    event_id: String,
    room_id: Option<String>,
    rel_type: Option<String>,
    event_type: Option<String>,
    limit: Option<u32>,
    from: Option<String>,
    to: Option<String>,
    direction: Option<ReadRelationsDirection>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReadRelationsResponse {
    chunk: Vec<MatrixEvent>,
    next_batch: String,
    prev_batch: String,
}
