use serde::{Deserialize, Serialize};

use super::{capabilities::Options, openid, MatrixEvent, MessageBody, Empty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ToWidgetMessage {
    #[serde(rename = "capabilities")]
    SendMeCapabilities(MessageBody<Empty, SendMeCapabilitiesResponse>),
    #[serde(rename = "notify_capabilities")]
    CapabilitiesUpdated(MessageBody<CapabilitiesUpdatedRequest, Empty>),
    #[serde(rename = "openid_credentials")]
    OpenIdCredentials(MessageBody<openid::State, Empty>),
    #[serde(rename = "send_event")]
    SendEvent(MessageBody<MatrixEvent, Empty>),
}

impl ToWidgetMessage {
    pub fn id(&self) -> &str {
        match self {
            ToWidgetMessage::SendMeCapabilities(MessageBody { header, .. })
            | ToWidgetMessage::CapabilitiesUpdated(MessageBody { header, .. })
            | ToWidgetMessage::OpenIdCredentials(MessageBody { header, .. })
            | ToWidgetMessage::SendEvent(MessageBody { header, .. }) => &header.request_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMeCapabilitiesResponse {
    pub capabilities: Options,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapabilitiesUpdatedRequest {
    pub requested: Options,
    pub approved: Options,
}
