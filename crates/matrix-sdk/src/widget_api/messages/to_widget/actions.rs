use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ToWidgetAction {
    #[serde(rename = "capabilities")]
    SendMeCapabilities,
    #[serde(rename = "notify_capabilities")]
    CapabilitiesUpdated
}
