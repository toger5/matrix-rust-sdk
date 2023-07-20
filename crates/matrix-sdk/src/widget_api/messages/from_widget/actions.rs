use serde::{Deserialize, Serialize};

use url::Url;

use crate::widget_api::messages::message::{Message, ActionBody};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum FromWidgetAction {
    #[serde(rename = "supported_api_versions")]
    GetSupportedApiVersion(ActionBody<(), SupportedVersions>),
    #[serde(rename = "content_loaded")]
    ContentLoaded(ActionBody<(), ()>),
    #[serde(rename = "org.matrix.msc2931.navigate")]
    Navigate(ActionBody<Url, Result<(), &'static str>>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportedVersions {
    pub versions: Vec<ApiVersion>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiVersion {
    PreRelease,
}
