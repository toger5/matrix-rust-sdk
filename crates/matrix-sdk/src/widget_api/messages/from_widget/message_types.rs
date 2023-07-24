use serde::{Deserialize, Serialize};

use url::Url;

use crate::widget_api::messages::{SupportedVersions};

use super::Request;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum FromWidgetMessage {
    #[serde(rename = "supported_api_versions")]
    GetSupportedApiVersion(Request<(), SupportedVersions>),
    #[serde(rename = "content_loaded")]
    ContentLoaded(Request<(), ()>),
    #[serde(rename = "org.matrix.msc2931.navigate")]
    Navigate(Request<Url, Result<(), &'static str>>),
}
