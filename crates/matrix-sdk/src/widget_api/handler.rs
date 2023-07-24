use async_trait::async_trait;

use super::{
    capabilities::{Capabilities},
    messages::{
        message::{MessageBody, Message},
        ApiVersion, CapabilitiesUpdated, ToWidget, SendMeCapabilities, SupportedVersions, SUPPORTED_API_VERSIONS, FromWidgetMessage,
    },
    Error, Result,
};

#[async_trait]
pub trait Driver {
    async fn initialise(&mut self, req: Capabilities) -> Result<Capabilities>;
    async fn send<T: ToWidget>(&mut self, req: T) -> Result<T::Response>;
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

    pub async fn handle(&mut self, req: FromWidgetMessage) -> Result<()> {
        match req {
            FromWidgetMessage::ContentLoaded(r) => {
                r.reply(())?;
                if self.capabilities.is_none() {
                    return Err(Error::WidgetError("Content loaded twice".to_string()));
                }

                let requested = self.driver.send(SendMeCapabilities).await?;
                let capabilities = self.driver.initialise(requested).await?;
                self.capabilities = Some(capabilities);

                let approved: CapabilitiesUpdated = self.capabilities.as_ref().unwrap().into();
                self.driver.send(approved).await?;
            }

            FromWidgetMessage::GetSupportedApiVersion(r) => {
                r.reply(SupportedVersions { versions: SUPPORTED_API_VERSIONS })?;
            }

            FromWidgetMessage::Navigate(r) => {
                match self.capabilities.as_ref().and_then(|c| c.navigate.as_ref()) {
                    Some(navigate) => {
                        navigate(r.content.clone());
                        r.reply(Ok(()))?;
                    }
                    None => {
                        r.reply(Err("Not permissions to call navigate"))?;
                    }
                }
            }
        }

        Ok(())
    }
}
