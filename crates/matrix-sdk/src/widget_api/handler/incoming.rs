use crate::widget_api::messages::Empty;

use super::{
    super::messages::{
        from_widget::{ReadEventRequest, ReadEventResponse, SendEventRequest, SendEventResponse},
        openid, SupportedVersions,
    },
    Request,
};

#[allow(missing_debug_implementations)]
pub enum Message {
    GetSupportedApiVersion(Request<Empty, SupportedVersions>),
    ContentLoaded(Request<Empty, Empty>),
    GetOpenID(Request<openid::Request, openid::State>),
    ReadEvents(Request<ReadEventRequest, ReadEventResponse>),
    SendEvent(Request<SendEventRequest, SendEventResponse>),
}
