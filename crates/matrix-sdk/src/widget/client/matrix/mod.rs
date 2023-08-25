use std::result::Result as StdResult;

use async_trait::async_trait;
use ruma::{
    api::client::{
        account::request_openid_token::v3::Request as MatrixOpenIDRequest, filter::RoomEventFilter,
    },
    events::AnySyncTimelineEvent,
    serde::Raw,
};
use tokio::sync::{mpsc, oneshot};

use super::handler::{
    Capabilities, Client, Error, EventReader as Reader, EventSender as Sender, Filtered,
    OpenIDDecision, OpenIDStatus, Result,
};
use crate::{
    event_handler::EventHandlerDropGuard,
    room::{Joined, MessagesOptions},
    widget::{
        messages::{
            from_widget::{
                ReadEventRequest, ReadEventResponse, SendEventRequest, SendEventResponse,
            },
            EventType, MatrixEvent, OpenIDRequest, OpenIDState,
        },
        EventFilter, Permissions, PermissionsProvider,
    },
};

#[derive(Debug)]
pub struct Driver<T> {
    room: Joined,
    permissions_provider: T,
    event_handler_handle: Option<EventHandlerDropGuard>,
}

impl<T> Driver<T> {
    pub fn new(room: Joined, permissions_provider: T) -> Self {
        Self { room, permissions_provider, event_handler_handle: None }
    }
}

#[async_trait]
impl<T: PermissionsProvider> Client for Driver<T> {
    async fn initialise(&mut self, permissions: Permissions) -> Capabilities {
        let permissions = self.permissions_provider.acquire_permissions(permissions).await;

        Capabilities {
            listener: Filters::new(permissions.read.clone())
                .map(|filters| self.setup_event_listener(filters)),
            reader: Filters::new(permissions.read).map(|filters| {
                let reader: Box<dyn Reader> =
                    Box::new(EventServerProxy::new(self.room.clone(), filters));
                reader
            }),
            sender: Filters::new(permissions.send).map(|filters| {
                let sender: Box<dyn Sender> =
                    Box::new(EventServerProxy::new(self.room.clone(), filters));
                sender
            }),
        }
    }

    fn get_openid(&self, req: OpenIDRequest) -> OpenIDStatus {
        let user_id = self.room.own_user_id().to_owned();
        let client = self.room.client.clone();
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let _ = tx.send(
                client
                    .send(MatrixOpenIDRequest::new(user_id), None)
                    .await
                    .map(|res| {
                        OpenIDDecision::Allowed(OpenIDState {
                            id: req.id,
                            token: res.access_token,
                            expires_in_seconds: res.expires_in.as_secs() as usize,
                            server: res.matrix_server_name.to_string(),
                            kind: res.token_type.to_string(),
                        })
                    })
                    .unwrap_or(OpenIDDecision::Blocked),
            );
        });

        OpenIDStatus::Pending(rx)
    }
}

impl<W> Driver<W> {
    fn setup_event_listener(&mut self, filter: Filters) -> mpsc::UnboundedReceiver<MatrixEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        let callback = move |ev: Raw<AnySyncTimelineEvent>| {
            let (filter, tx) = (filter.clone(), tx.clone());
            if let Ok(msg) = ev.deserialize_as::<MatrixEvent>() {
                filter.allow(&msg.event_type).then(|| tx.send(msg));
            }
            async {}
        };

        let handle = self.room.add_event_handler(callback);
        let drop_guard = self.room.client().event_handler_drop_guard(handle);
        self.event_handler_handle.replace(drop_guard);
        rx
    }
}

#[derive(Debug)]
pub struct EventServerProxy {
    room: Joined,
    filter: Filters,
}

impl EventServerProxy {
    fn new(room: Joined, filter: Filters) -> Self {
        Self { room, filter }
    }
}

#[async_trait]
impl Reader for EventServerProxy {
    async fn read(&self, req: ReadEventRequest) -> Result<ReadEventResponse> {
        let options = {
            let mut o = MessagesOptions::backward();
            o.limit = req.limit.into();
            o.filter = {
                let mut f = RoomEventFilter::default();
                f.types = Some(vec![req.event_type.event_type().to_string()]);
                f
            };
            o
        };

        // Fetch messages from the server.
        let messages = self.room.messages(options).await.map_err(Error::other)?;

        // Iterator over deserialized state messages.
        let state = messages.state.into_iter().map(|s| s.deserialize_as::<MatrixEvent>());

        // Iterator over deserialized timeline messages.
        let timeline = messages.chunk.into_iter().map(|m| m.event.deserialize_as());

        // Chain two iterators together and run them through the filter.
        Ok(ReadEventResponse {
            events: state
                .chain(timeline)
                .filter_map(StdResult::ok)
                .filter(|m| self.filter.allow(&m.event_type))
                .collect(),
        })
    }
}

#[async_trait]
impl Sender for EventServerProxy {
    async fn send(&self, req: SendEventRequest) -> Result<SendEventResponse> {
        // Run the request through the filter.
        if !self.filter.allow(&req.event_type) {
            return Err(Error::custom("Message not allowed by filter"));
        }

        // Send the request based on whether the state key is set or not.
        let event_id = match req.event_type {
            EventType::State { event_type, state_key } => {
                self.room
                    .send_state_event_raw(req.content, &event_type.to_string(), &state_key)
                    .await
                    .map_err(Error::other)?
                    .event_id
            }
            EventType::MessageLike { event_type, .. } => {
                self.room
                    .send_raw(req.content, &event_type.to_string(), None)
                    .await
                    .map_err(Error::other)?
                    .event_id
            }
        };

        // Return the response.
        Ok(SendEventResponse {
            room_id: self.room.room_id().to_string(),
            event_id: event_id.to_string(),
        })
    }
}

impl Filtered for EventServerProxy {
    fn filters(&self) -> &[EventFilter] {
        &self.filter.filters
    }
}

#[derive(Debug, Clone)]
struct Filters {
    filters: Vec<EventFilter>,
}

impl Filters {
    fn new(filters: Vec<EventFilter>) -> Option<Self> {
        (!filters.is_empty()).then_some(Self { filters })
    }

    fn allow(&self, event_type: &EventType) -> bool {
        self.filters.iter().any(|f| event_type.matches(f))
    }
}
