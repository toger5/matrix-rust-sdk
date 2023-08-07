use async_trait::async_trait;

use super::handler::Result;
use crate::widget_api::messages::capabilities;

// the Widget trait that needs to be implemented in the native client
#[async_trait]
pub trait Widget {
    /// This is the most important message to be implemented in the client driver
    /// the function will be called on the struct implementing the client driver whenever there is a new message available
    /// that has to be sent to the widget
    fn send_widget_message(&self, message: &str) -> Result<()>;

    async fn show_capability_dialog(
        &self,
        cap: capabilities::Options,
    ) -> Result<capabilities::Options>;
    async fn show_get_openid_dialog(&self) -> Result<bool>;
    fn id(&self) -> &str;
    fn get_widget_state_json(&self) -> &str;
}

#[derive(Debug)]
pub struct DummyWidget {}

#[async_trait]
impl Widget for DummyWidget {
    fn send_widget_message(&self, message: &str) -> Result<()> {
        todo!()
    }

    async fn show_capability_dialog(
        &self,
        cap: capabilities::Options,
    ) -> Result<capabilities::Options> {
        Ok(cap)
    }
    async fn show_get_openid_dialog(&self) -> Result<bool> {
        Ok(true)
    }

    fn id(&self) -> &str {
        todo!()
    }

    fn get_widget_state_json(&self) -> &str {
        todo!()
    }
}
