use crate::widget::messages::{
    to_widget::{Action, CapabilitiesResponse, CapabilitiesUpdatedRequest},
    Empty, MessageKind as Kind, OpenIDResponse,
};

pub type Response<T> = Result<T, String>;

pub trait Request: Sized + Send + Sync + 'static {
    type Response;

    fn into_action(self) -> Action;
    fn extract_response(reply: Action) -> Option<Response<Self::Response>>;
}

pub struct CapabilitiesRequest;
impl Request for CapabilitiesRequest {
    type Response = CapabilitiesResponse;

    fn into_action(self) -> Action {
        Action::CapabilitiesRequest(Kind::empty())
    }

    fn extract_response(reply: Action) -> Option<Response<Self::Response>> {
        match reply {
            Action::CapabilitiesRequest(Kind::Response(r)) => Some(r.response()),
            _ => None,
        }
    }
}

pub struct CapabilitiesUpdate(pub CapabilitiesUpdatedRequest);
impl Request for CapabilitiesUpdate {
    type Response = Empty;

    fn into_action(self) -> Action {
        Action::CapabilitiesUpdate(Kind::request(self.0))
    }

    fn extract_response(reply: Action) -> Option<Response<Self::Response>> {
        match reply {
            Action::CapabilitiesUpdate(Kind::Response(r)) => Some(r.response()),
            _ => None,
        }
    }
}

pub struct OpenIDUpdated(pub OpenIDResponse);
impl Request for OpenIDUpdated {
    type Response = Empty;

    fn into_action(self) -> Action {
        Action::OpenIdCredentialsUpdate(Kind::request(self.0))
    }

    fn extract_response(reply: Action) -> Option<Response<Self::Response>> {
        match reply {
            Action::OpenIdCredentialsUpdate(Kind::Response(r)) => Some(r.response()),
            _ => None,
        }
    }
}
