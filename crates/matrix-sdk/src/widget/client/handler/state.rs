use std::sync::Arc;

use tokio::sync::mpsc::UnboundedReceiver;
use tracing::{info, warn};

use super::{
    outgoing, Capabilities, Client, Error, IncomingRequest as Request, OpenIDResponse,
    OpenIDStatus, Result, WidgetProxy,
};
use crate::widget::{
    messages::{
        from_widget::{ApiVersion, SupportedApiVersionsResponse},
        to_widget::{CapabilitiesResponse, CapabilitiesUpdatedRequest},
        Empty,
    },
    Permissions,
};

pub struct State<W, C> {
    capabilities: Option<Capabilities>,
    widget: Arc<W>,
    client: C,
}

impl<W, C> State<W, C> {
    pub fn new(widget: Arc<W>, client: C) -> Self {
        Self { capabilities: None, widget, client }
    }
}

pub enum Task {
    NegotiateCapabilities,
    HandleIncoming(Request),
}

impl<W: WidgetProxy, C: Client> State<W, C> {
    pub async fn listen(mut self, mut rx: UnboundedReceiver<Task>) {
        while let Some(msg) = rx.recv().await {
            match msg {
                Task::HandleIncoming(request) => {
                    if let Err(err) = self.handle(request.clone()).await {
                        if let Err(..) = self.widget.reply(request.fail(err.to_string())) {
                            info!("Dropped reply, widget is disconnected");
                            break;
                        }
                    }
                }
                Task::NegotiateCapabilities => {
                    if let Err(err) = self.initialise().await {
                        // We really don't have a mechanism to inform a widget about out of bound
                        // errors. So the only thing we can do here is to log it.
                        warn!(error = %err, "Failed to initialise widget");
                        break;
                    }
                }
            }
        }
    }

    async fn handle(&mut self, request: Request) -> Result<()> {
        match request {
            Request::GetSupportedApiVersion(req) => {
                let _ = self.widget.reply(req.map(Ok(SupportedApiVersionsResponse::new())));
            }

            Request::ContentLoaded(req) => {
                let (response, negotiate) =
                    match (self.widget.init_on_load(), self.capabilities.as_ref()) {
                        (true, None) => (Ok(Empty {}), true),
                        (true, Some(..)) => (Err("Already loaded".into()), false),
                        _ => (Ok(Empty {}), false),
                    };

                let _ = self.widget.reply(req.map(response));
                if negotiate {
                    self.initialise().await?;
                }
            }

            Request::GetOpenID(req) => {
                let (reply, handle) = match self.client.get_openid((*req).clone()) {
                    OpenIDStatus::Resolved(decision) => (decision.into(), None),
                    OpenIDStatus::Pending(handle) => (OpenIDResponse::Pending, Some(handle)),
                };

                let _ = self.widget.reply(req.map(Ok(reply)));
                if let Some(handle) = handle {
                    let status = handle.await.map_err(|_| Error::WidgetDisconnected)?;
                    self.widget
                        .send(outgoing::OpenIDUpdated(status.into()))
                        .await?
                        .map_err(Error::WidgetErrorReply)?;
                }
            }

            Request::ReadEvent(req) => {
                let fut = self
                    .caps()?
                    .reader
                    .as_ref()
                    .ok_or(Error::custom("No permissions to read events"))?
                    .read((*req).clone());
                let resp = Ok(fut.await?);
                let _ = self.widget.reply(req.map(resp));
            }

            Request::SendEvent(req) => {
                let fut = self
                    .caps()?
                    .sender
                    .as_ref()
                    .ok_or(Error::custom("No permissions to send events"))?
                    .send((*req).clone());
                let resp = Ok(fut.await?);
                let _ = self.widget.reply(req.map(resp));
            }
        }

        Ok(())
    }

    async fn initialise(&mut self) -> Result<()> {
        let CapabilitiesResponse { capabilities: desired } = self
            .widget
            .send(outgoing::CapabilitiesRequest)
            .await?
            .map_err(Error::WidgetErrorReply)?;

        let capabilities = self.client.initialise(desired.clone()).await;
        let approved: Permissions = (&capabilities).into();
        self.capabilities = Some(capabilities);

        let update = CapabilitiesUpdatedRequest { requested: desired, approved };
        self.widget
            .send(outgoing::CapabilitiesUpdate(update))
            .await?
            .map_err(Error::WidgetErrorReply)?;

        Ok(())
    }

    fn caps(&mut self) -> Result<&mut Capabilities> {
        self.capabilities.as_mut().ok_or(Error::custom("Capabilities have not been negotiated"))
    }
}

impl SupportedApiVersionsResponse {
    pub fn new() -> Self {
        Self {
            versions: vec![
                ApiVersion::V0_0_1,
                ApiVersion::V0_0_2,
                ApiVersion::MSC2762,
                ApiVersion::MSC2871,
                ApiVersion::MSC3819,
            ],
        }
    }
}
