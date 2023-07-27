use super::{capabilities::Capabilities, handler, handler::Outgoing, messages, widget::Widget};
use async_trait::async_trait;
use super::error::Result;

pub use self::matrix_driver::RustSdkMatrixDriver;

pub mod matrix_driver;

pub struct Driver<W: Widget> {
    pub matrix_driver: RustSdkMatrixDriver,
    pub widget: W,
}
#[async_trait]
impl<W: Widget> handler::Driver for Driver<W> {
    async fn initialise(&mut self, req: messages::capabilities::Options) -> Result<Capabilities> {
        todo!()
    }
    async fn send(&mut self, message: Outgoing) -> Result<()> {
        todo!()
    }
}
