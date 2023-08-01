use super::{
    super::messages::{openid, SupportedVersions},
    Request,
};

#[allow(missing_debug_implementations)]
pub enum Message {
    GetSupportedApiVersion(Request<(), SupportedVersions>),
    ContentLoaded(Request<(), ()>),
    GetOpenID(Request<openid::Request, openid::State>),
}
