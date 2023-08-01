mod incoming;
mod outgoing;
mod request;

use super::{
    capabilities::Capabilities,
    messages::{
        capabilities::Options as CapabilitiesReq, SupportedVersions, SUPPORTED_API_VERSIONS,
    },
};

pub use self::{incoming::Message as Incoming, outgoing::Message as Outgoing, request::Request};
pub use super::{Error, Result};

// Make this an async trait wiht async initialise and send functions
pub trait Driver {
    fn initialise(&self, req: CapabilitiesReq) -> Result<Capabilities>;
    fn send(&self, message: Outgoing) -> Result<()>;
}

#[allow(missing_debug_implementations)]
pub struct MessageHandler<T> {
    capabilities: Option<Capabilities>,
    driver: T,
}

impl<T: Driver> MessageHandler<T> {
    pub fn new(driver: T) -> Self {
        Self { capabilities: None, driver }
    }

    pub async fn handle(&mut self, req: Incoming) -> Result<()> {
        match req {
            Incoming::ContentLoaded(r) => {
                r.reply(())?;
                if self.capabilities.is_none() {
                    return Err(Error::WidgetError("Content loaded twice".to_string()));
                }

                let (req, resp) = Request::new(());
                self.driver.send(Outgoing::SendMeCapabilities(req))?;//.await?;
                let options = resp.await.map_err(|_| Error::WidgetDied)?;

                let capabilities = self.driver.initialise(options)?;//.await?;
                self.capabilities = Some(capabilities);

                let approved: CapabilitiesReq = self.capabilities.as_ref().unwrap().into();
                let (req, resp) = Request::new(approved);
                self.driver.send(Outgoing::CapabilitiesUpdated(req));//.await?;
                resp.await.map_err(|_| Error::WidgetDied)?;
            }

            Incoming::GetSupportedApiVersion(r) => {
                r.reply(SupportedVersions { versions: SUPPORTED_API_VERSIONS.to_vec() })?;
            }
        }

        Ok(())
    }
}
