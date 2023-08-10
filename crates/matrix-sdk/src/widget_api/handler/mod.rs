use std::result::Result as StdResult;

pub use self::{
    capabilities::{Capabilities, EventReader, EventSender},
    driver::{Driver, OpenIDResult, OpenIDState},
    incoming::Message as Incoming,
    outgoing::Message as Outgoing,
    request::{Request, Response},
};
use super::{
    messages::{
        self,
        capabilities::Options as CapabilitiesReq,
        from_widget::{ReadEventRequest, ReadEventResponse, SendEventRequest, SendEventResponse},
        openid, SupportedVersions, SUPPORTED_API_VERSIONS,
    },
    Error, Result,
};

mod capabilities;
mod driver;
mod incoming;
mod outgoing;
mod request;

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
                    None => Err("Capabilities have already been sent".to_owned()),
                };
                r.reply(response)?;
                self.initialise().await?;
            }

            Incoming::GetSupportedApiVersion(r) => {
                r.reply(Ok(SupportedVersions { versions: SUPPORTED_API_VERSIONS.to_vec() }))?;
            }

            Incoming::GetOpenID(r) => {
                let state = self.driver.get_openid(r.clone()).await;
                r.reply(
                    (&state)
                        .as_ref()
                        .map(|s| -> openid::State { s.into() })
                        .map_err(|e| e.to_string()),
                )?;

                if let Ok(OpenIDState::Pending(resolution)) = state {
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

            Incoming::SendEvent(r) => {
                let response = self.send_event(&r).await;
                r.reply(response)?;
            }

            Incoming::SendToDeviceRequest(_r) => {
                unimplemented!()
            }
        }

        Ok(())
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

    async fn read_events(
        &mut self,
        req: &ReadEventRequest,
    ) -> StdResult<ReadEventResponse, String> {
        self.capabilities()?
            .event_reader
            .as_mut()
            .ok_or("No permissions to read the events".to_owned())?
            .read(req.clone())
            .await
            .map_err(|_| "Failed to read events".to_owned())
    }

    async fn send_event(&mut self, req: &SendEventRequest) -> StdResult<SendEventResponse, String> {
        self.capabilities()?
            .event_sender
            .as_mut()
            .ok_or("No permissions to write the events".to_owned())?
            .send(req.clone())
            .await
            .map_err(|_| "Failed to write events".to_owned())
    }

    fn capabilities(&mut self) -> StdResult<&mut Capabilities, String> {
        self.capabilities.as_mut().ok_or("Capabilities have not been negotiated".to_owned())
    }
}
