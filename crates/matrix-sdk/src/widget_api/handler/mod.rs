use std::result::Result as StdResult;

mod driver;
mod incoming;
mod outgoing;
mod request;

pub use self::{
    driver::{Driver, OpenIDState},
    incoming::Message as Incoming,
    outgoing::Message as Outgoing,
    request::Request,
};
use super::{
    capabilities::{Capabilities, ReadEventRequest},
    messages::{
        capabilities::Options as CapabilitiesReq, MatrixEvent, SupportedVersions,
        SUPPORTED_API_VERSIONS,
    },
};
pub use super::{Error, Result};

#[allow(missing_debug_implementations)]
pub struct MessageHandler<T> {
    capabilities: Option<Capabilities>,
    driver: T,
}

impl<T: Driver> MessageHandler<T> {
    pub async fn new(driver: T, init_immediately: bool) -> Result<Self> {
        let mut handler = Self { capabilities: None, driver };
        if init_immediately {
            handler.initialise().await?;
        }

        Ok(handler)
    }

    pub async fn handle(&mut self, req: Incoming) -> Result<()> {
        match req {
            Incoming::ContentLoaded(r) => {
                let response = match self.capabilities.as_ref() {
                    Some(..) => Ok(()),
                    None => Err("Capabilities have already been sent"),
                };
                r.reply(response)?;
                self.initialise().await?;
            }

            Incoming::GetSupportedApiVersion(r) => {
                r.reply(Ok(SupportedVersions { versions: SUPPORTED_API_VERSIONS.to_vec() }))?;
            }

            Incoming::GetOpenID(r) => {
                let state = self.driver.get_openid(r.clone()).await;
                r.reply(Ok((&state).into()))?;

                if let OpenIDState::Pending(resolution) = state {
                    let resolved = resolution.await.map_err(|_| Error::WidgetDied)?;
                    let (req, resp) = Request::new(resolved.into());
                    self.driver.send(Outgoing::OpenIDUpdated(req)).await?;
                    resp.recv().await?;
                }
            }

            Incoming::ReadEvents(r) => {
                let response = self.read_events(&r).await;
                r.reply(response)?;
            }
        }

        Ok(())
    }

    async fn read_events(&mut self, req: &ReadEventRequest) -> StdResult<Vec<MatrixEvent>, &'static str> {
        let events = self.capabilities
            .as_mut()
            .ok_or("Capabilities have not been negotiated")?
            .event_reader
            .as_mut()
            .ok_or("No permissions to read the events")?
            .read(req.clone())
            .await;
        Ok(events)
    }

    async fn initialise(&mut self) -> Result<()> {
        let (req, resp) = Request::new(());
        self.driver.send(Outgoing::SendMeCapabilities(req)).await?;
        let options = resp.recv().await?;

        let capabilities = self.driver.initialise(options).await?;
        self.capabilities = Some(capabilities);

        let approved: CapabilitiesReq = self.capabilities.as_ref().unwrap().into();
        let (req, resp) = Request::new(approved);
        self.driver.send(Outgoing::CapabilitiesUpdated(req)).await?;
        resp.recv().await?;

        Ok(())
    }
}
