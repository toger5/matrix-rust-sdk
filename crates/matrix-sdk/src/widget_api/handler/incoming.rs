use super::{
    messages::{
        from_widget::{
            ReadEventRequest, ReadEventResponse, SendEventRequest, SendEventResponse,
            SendToDeviceRequest,
        },
        openid, SupportedVersions,
    },
    Request,
};

#[allow(missing_debug_implementations)]
pub enum Message {
    GetSupportedApiVersion(Request<(), SupportedVersions>),
    ContentLoaded(Request<(), ()>),
    GetOpenID(Request<openid::Request, openid::State>),
    ReadEvents(Request<ReadEventRequest, ReadEventResponse>),
    SendEvent(Request<SendEventRequest, SendEventResponse>),
    SendToDeviceRequest(Request<SendToDeviceRequest, ()>),
}