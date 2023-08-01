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
}
