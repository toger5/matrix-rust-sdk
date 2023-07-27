
use super::{handler, driver::{RustSdkMatrixDriver, Driver}, widget::Widget};

pub trait Transport{
    fn receive(&self, message: &str);
    fn on_incoming(&self, handle_incoming: Box<dyn Fn(handler::Incoming)>);
}  

pub struct DummyTransport {
}
impl Transport for DummyTransport {
    fn receive(&self, message: &str) {
        println!("Dummy transport is received message: {}", message)
    }
    fn on_incoming(&self, handle_incoming: Box<dyn Fn(handler::Incoming)>) {
        unimplemented!()
    }
}
