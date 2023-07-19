use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::widgets::widget_message::WidgetActionBody;

use super::WidgetMessageEmptyData;
#[derive(Serialize, Deserialize, Debug)]
struct ToWidgetSendToDeviceRequestBody {
    #[serde(rename = "type")]
    message_type: String,
    sender: String,
    encrypted: bool,
    messages: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct FromWidgetSendToDeviceRequest {
    #[serde(rename = "type")]
    message_type: String,
    encrypted: bool,
    content: HashMap<String, HashMap<String, serde_json::Value>>,
}

pub type ToWidgetSendToDeviceBody =
    WidgetActionBody<ToWidgetSendToDeviceRequestBody, WidgetMessageEmptyData>;
pub type FromWidgetSendToDeviceBody =
    WidgetActionBody<FromWidgetSendToDeviceRequest, WidgetMessageEmptyData>;
