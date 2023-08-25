use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot::Receiver,
};

use self::state::{State, Task as StateTask};
pub use self::{
    capabilities::{Capabilities, EventReader, EventSender, Filtered},
    error::{Error, Result},
    outgoing::{Request as Outgoing, Response},
    state::IncomingRequest,
};
use crate::widget::{
    messages::{
        from_widget::{Action, SupportedApiVersionsResponse},
        Header, MessageKind, OpenIDRequest, OpenIDResponse, OpenIDState,
    },
    Permissions,
};

mod capabilities;
mod error;
mod outgoing;
mod state;

#[allow(missing_debug_implementations)]
pub struct MessageHandler<W> {
    state_tx: UnboundedSender<StateTask>,
    widget: Arc<W>,
}

impl<W: WidgetProxy> MessageHandler<W> {
    pub fn new(client: impl Client, widget: W) -> Self {
        let widget = Arc::new(widget);

        let (state_tx, state_rx) = unbounded_channel();
        tokio::spawn(State::new(widget.clone(), client).listen(state_rx));

        if !widget.init_on_load() {
            let _ = state_tx.send(StateTask::NegotiateCapabilities);
        }

        Self { widget, state_tx }
    }

    pub fn handle(&self, req: IncomingRequest) -> Result<()> {
        match req.action {
            Action::GetSupportedApiVersion(MessageKind::Request(r)) => {
                let response = r.map(Ok(SupportedApiVersionsResponse::new()));
                self.widget
                    .reply(Reply::new(req.header, Action::GetSupportedApiVersion(response)))?;
            }

            _ => {
                self.state_tx
                    .send(StateTask::HandleIncoming(req))
                    .map_err(|_| Error::WidgetDisconnected)?;
            }
        }

        Ok(())
    }
}

#[async_trait]
pub trait WidgetProxy: Send + Sync + 'static {
    async fn send<T: Outgoing>(&self, req: T) -> Result<Response<T::Response>>;
    fn reply(&self, reply: Reply) -> Result<()>;
    fn init_on_load(&self) -> bool;
}

pub struct Reply {
    pub header: Header,
    pub action: Action,
}

impl Reply {
    pub fn new(header: Header, action: Action) -> Self {
        Self { header, action }
    }
}

#[async_trait]
pub trait Client: Send + Sync + 'static {
    async fn initialise(&mut self, req: Permissions) -> Capabilities;
    fn get_openid(&self, req: OpenIDRequest) -> OpenIDStatus;
}

#[derive(Debug)]
pub enum OpenIDStatus {
    #[allow(dead_code)]
    Resolved(OpenIDDecision),
    Pending(Receiver<OpenIDDecision>),
}

#[derive(Debug, Clone)]
pub enum OpenIDDecision {
    Blocked,
    Allowed(OpenIDState),
}

impl From<OpenIDDecision> for OpenIDResponse {
    fn from(decision: OpenIDDecision) -> Self {
        match decision {
            OpenIDDecision::Allowed(resolved) => OpenIDResponse::Allowed(resolved),
            OpenIDDecision::Blocked => OpenIDResponse::Blocked,
        }
    }
}
