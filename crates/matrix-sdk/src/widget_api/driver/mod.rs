use super::error::Result;
use super::messages::capabilities::Options;
use super::{capabilities::Capabilities, handler, handler::Outgoing,  widget::Widget};
use async_trait::async_trait;
pub use self::matrix_driver::RustSdkMatrixDriver;

pub mod matrix_driver;

pub struct Driver<W: Widget> {
    pub matrix_driver: RustSdkMatrixDriver,
    pub widget: W,
}
impl<W: Widget> handler::Driver for Driver<W> {
    // should be async
    fn send(&self, message: Outgoing) -> Result<()>{
        Result::Ok(())
    }
    // should be async
    fn initialise(&self, req: Options) -> Result<Capabilities>{
        Result::Ok(Capabilities::default())
    }
}
