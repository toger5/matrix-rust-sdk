use crate::widget_api::{capabilities::SendEventRequest, messages::from_widget::SendEventResponse};

use super::{
    super::{
        capabilities::ReadEventRequest,
        messages::{openid, SupportedVersions, MatrixEvent},
    },
    Request,
};

#[allow(missing_debug_implementations)]
pub enum Message {
    GetSupportedApiVersion(Request<(), SupportedVersions>),
    ContentLoaded(Request<(), ()>),
    GetOpenID(Request<openid::Request, openid::State>),
    ReadEvents(Request<ReadEventRequest, Vec<MatrixEvent>>),
    SendEvent(Request<SendEventRequest, SendEventResponse>),
}
