//! Client widget API implementation.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use serde_json::{from_str as from_json, to_string as to_json};
use tokio::sync::{mpsc::UnboundedSender, oneshot};
use uuid::Uuid;

use self::handler::{
    Client, Error, IncomingRequest, MessageHandler, Outgoing, Reply, Response, WidgetProxy,
};
pub use self::matrix::Driver as MatrixDriver;
use super::{
    messages::{to_widget::Action as ToWidgetAction, Action, Header, Message},
    Info as WidgetInfo, Widget,
};

mod handler;
mod matrix;

pub type PendingResponses = Arc<Mutex<HashMap<String, oneshot::Sender<ToWidgetAction>>>>;

/// Runs client widget API handler with a given widget. Returns once the widget
/// is closed.
pub async fn run<T: Client>(client: T, mut widget: Widget) {
    // State: a map of outgoing requests that are waiting response from a widget.
    let state: PendingResponses = Arc::new(Mutex::new(HashMap::new()));

    // Create a message handler (handles all incoming requests and generates
    // outgoing requests).
    let handler = {
        let widget = WidgetSink::new(widget.info, widget.comm.to, state.clone());
        MessageHandler::new(client, widget)
    };

    // Receives plain JSON string messages from a widget and passes them
    // to the message processor that forwards them to the required handlers.
    while let Some(raw) = widget.comm.from.recv().await {
        match from_json::<Message>(&raw) {
            Ok(msg) => match msg.action {
                Action::FromWidget(a) => {
                    let _ = handler.handle(IncomingRequest { header: msg.header, action: a });
                }
                Action::ToWidget(a) => {
                    let maybe_reply = state
                        .lock()
                        .expect("Pending mutex poisoned")
                        .remove(&msg.header.request_id);
                    if let Some(reply) = maybe_reply {
                        let _ = reply.send(a);
                    }
                }
            },
            Err(err) => {
                todo!("Handle invalid JSON message: {}", err);
            }
        }
    }
}

struct WidgetSink {
    info: WidgetInfo,
    sink: UnboundedSender<String>,
    pending: PendingResponses,
}

impl WidgetSink {
    fn new(info: WidgetInfo, sink: UnboundedSender<String>, pending: PendingResponses) -> Self {
        Self { info, sink, pending }
    }
}

#[async_trait]
impl WidgetProxy for WidgetSink {
    async fn send<T: Outgoing>(&self, msg: T) -> Result<Response<T::Response>, Error> {
        let id = Uuid::new_v4().to_string();
        let header = Header::new(&id, &self.info.id);
        let action = Action::ToWidget(msg.into_action());
        let message = Message { header, action };
        let json = to_json(&message).expect("Bug: failed to serialise a message");
        self.sink.send(json).map_err(|_| Error::WidgetDisconnected)?;

        let (tx, rx) = oneshot::channel();
        self.pending.lock().expect("Pending mutex poisoned").insert(id.to_string(), tx);
        let reply = rx.await.map_err(|_| Error::WidgetDisconnected)?;
        Ok(T::extract_response(reply).ok_or(Error::custom("Widget sent invalid response"))?)
    }

    fn reply(&self, reply: Reply) -> Result<(), Error> {
        let message = Message { header: reply.header, action: Action::FromWidget(reply.action) };
        let json = to_json(&message).expect("Bug: failed to serialise a message");
        self.sink.send(json).map_err(|_| Error::WidgetDisconnected)
    }

    fn init_on_load(&self) -> bool {
        self.info.init_on_load
    }
}
