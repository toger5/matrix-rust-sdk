use serde::{Deserialize, Serialize};

use crate::widgets::widget_message::WidgetActionBody;

#[derive(Serialize, Deserialize, Debug)]
struct ToWidgetSendToDeviceRequest {
    #[serde(rename = "type")]
    message_type: String,
    encrypted: bool,
    // messages: { [userId: string]: { [deviceId: string]: object } };
}

#[derive(Serialize, Deserialize, Debug)]
struct ToWidgetSendToDeviceResponse {
    // nothing
}

#[derive(Serialize, Deserialize, Debug)]
struct FromWidgetSendToDeviceRequest {
    encrypted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FromWidgetSendToDeviceResponse {
    // nothing
}

pub type ToWidgetSendToDeviceBody =
    WidgetActionBody<ToWidgetSendToDeviceRequest, ToWidgetSendToDeviceResponse>;
pub type FromWidgetSendToDeviceBody =
    WidgetActionBody<FromWidgetSendToDeviceRequest, FromWidgetSendToDeviceResponse>;
