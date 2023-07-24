use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportedVersions {
    pub versions: Vec<ApiVersion>,
}

pub static SUPPORTED_API_VERSIONS: Vec<ApiVersion> = vec![
    ApiVersion::V0_0_1,
    ApiVersion::V0_0_2,
    ApiVersion::MSC2762,
    ApiVersion::MSC2871,
    ApiVersion::MSC3819,
    ApiVersion::MSC3869,
    ];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiVersion {
    #[serde(rename = "0.0.1")]
    V0_0_1,
    #[serde(rename = "0.0.2")]
    V0_0_2,
    #[serde(rename = "org.matrix.msc2762")] // Allowing widgets to send/receive events
    MSC2762,
    #[serde(rename = "org.matrix.msc2871")] // Sending approved capabilities back to the widget
    MSC2871,
    #[serde(rename = "org.matrix.msc2931")] // Allow widgets to navigate with matrix.to URIs 
    MSC2931,
    #[serde(rename = "org.matrix.msc2974")] // Widgets: Capabilities re-exchange 
    MSC2974,
    #[serde(rename = "org.matrix.msc2876")] // Allowing widgets to read events in a room (Closed/Deprecated)
    MSC2876,
    #[serde(rename = "org.matrix.msc3819")] // Allowing widgets to send/receive to-device messages
    MSC3819,
    #[serde(rename = "town.robin.msc3846")] // Allowing widgets to access TURN servers 
    MSC3846,
    #[serde(rename = "org.matrix.msc3869")] // Read event relations with the Widget API 
    MSC3869,
}

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
pub struct MatrixEvent {
    #[serde(rename = "type")]
    event_type: String,
    sender: String,
    event_id: String,
    room_id: String,
    state_key: Option<String>,
    origin_server_ts: u32,
    content: serde_json::Value,
    unsigned: Unsigned,
}

#[derive(Serialize, Deserialize, Debug)]
struct Unsigned {
    age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReadRelationsDirection {
    #[serde(rename = "f")]
    Forwards,
    #[serde(rename = "b")]
    Backwards
}