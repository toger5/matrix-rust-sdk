use async_trait::async_trait;
use tokio::sync::oneshot::Receiver;

use super::{
    super::messages::{capabilities::Options as CapabilitiesReq, openid},
    capabilities::Capabilities,
    OutgoingMessage, Result,
};

#[async_trait]
pub trait Client: Send + Sync + 'static {
    async fn initialise(&self, req: CapabilitiesReq) -> Result<Capabilities>;
    async fn get_openid(&self, req: openid::Request) -> OpenIDState;
}

#[async_trait]
pub trait Widget: Send + Sync + 'static {
    async fn send<T: OutgoingMessage>(&self, message: T) -> Result<T::Response>;
    fn early_init(&self) -> bool;
}

#[derive(Debug)]
pub enum OpenIDState {
    Resolved(OpenIDResult),
    Pending(Receiver<OpenIDResult>),
}

pub type OpenIDResult = Result<openid::Response>;

impl<'t> From<&'t OpenIDState> for openid::State {
    fn from(state: &'t OpenIDState) -> Self {
        match state {
            OpenIDState::Resolved(resolved) => resolved.clone().into(),
            OpenIDState::Pending(..) => openid::State::Pending,
        }
    }
}

impl From<OpenIDResult> for openid::State {
    fn from(result: OpenIDResult) -> Self {
        match result {
            Ok(response) => openid::State::Allowed(response),
            Err(_) => openid::State::Blocked,
        }
    }
}
