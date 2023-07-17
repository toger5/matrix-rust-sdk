use std::time::Instant;

use futures_util::future::BoxFuture;
use tokio::sync::mpsc::{Receiver, Sender};
use url::Url;

use crate::Result;

#[derive(Debug, Default)]
pub struct Request {
    read: bool,
    send: bool,
    navigate: bool,
}

type CapabilityFuture<T> = BoxFuture<'static, Result<T>>;
type NavigateFunc = Box<dyn Fn(Url) -> CapabilityFuture<()>>;
type OpenIDFunc = Box<dyn Fn() -> CapabilityFuture<OpenIDCredentials>>;

pub struct Capabilities {
    pub navigate: Option<NavigateFunc>,
    pub events: Option<Receiver<Event>>,
    pub send: Option<Sender<Event>>,
    pub acquire_token: Option<OpenIDFunc>,
}

// TODO: Replace it with the actual events that we can get.
#[derive(Debug)]
pub enum Event {
    RoomEvent,
    ToDeviceMessage,
}

#[derive(Debug)]
pub struct OpenIDCredentials {
    pub token: String,
    pub kind: TokenKind,
    pub expires: Instant,
    pub homeserver: Url,
}

#[derive(Debug)]
pub enum TokenKind {
    Bearer,
    Custom(String),
}
