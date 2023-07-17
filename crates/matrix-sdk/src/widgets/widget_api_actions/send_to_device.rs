use serde::{Serialize, Deserialize};

use crate::widgets::widget_message::WidgetActionBody;

#[derive(Serialize, Deserialize, Debug)]
pub struct SendToDeviceRequest {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendToDeviceResponse{
    
}

pub type SendToDeviceBody = WidgetActionBody<SendToDeviceRequest, SendToDeviceResponse>;