use async_trait::async_trait;
use ruma::{
    api::client::{account::request_openid_token, filter::RoomEventFilter},
    events::AnySyncTimelineEvent,
    serde::Raw,
};
use tokio::sync::mpsc;

use self::widget::Widget;
use super::{
    handler::{self, Capabilities, OpenIDState},
    messages::{
        self,
        capabilities::{EventFilter, Filter, Options},
        from_widget::{ReadEventRequest, ReadEventResponse, SendEventRequest, SendEventResponse},
        {openid, MatrixEvent},
    },
    {Error::WidgetError, Result},
};
use crate::{
    room::MessagesOptions,
    {event_handler::EventHandlerHandle, room::Joined},
};

pub mod widget;

#[derive(Debug)]
pub struct Driver<W: Widget> {
    room: Joined,
    widget: W,
    // TODO disconnect the handle if the driver is dropped.
    add_event_handler_handle: Option<EventHandlerHandle>,
}

#[async_trait(?Send)]
impl<W: Widget> handler::Driver for Driver<W> {
    async fn send(&self, _message: handler::Outgoing) -> Result<()> {
        unimplemented!()
    }

    async fn initialise(&mut self, options: Options) -> Result<Capabilities> {
        let options = self.widget.aquire_permissions(options).await?;

        Result::Ok(Capabilities {
            event_listener: self.build_event_listener(&options.read_filter),
            event_sender: {
                let filter = &options.send_filter;
                if filter.is_empty() {
                    None
                } else {
                    Some(Box::new(EventSender { room: self.room.clone(), filter: filter.clone() }))
                }
            },
            event_reader: {
                let filter = &options.read_filter;
                if filter.is_empty() {
                    None
                } else {
                    Some(Box::new(EventReader { room: self.room.clone(), filter: filter.clone() }))
                }
            },
            ..Capabilities::default()
        })
    }

    async fn get_openid(&self, req: openid::Request) -> OpenIDState {
        // TODO: make the client ask the user first.
        // We wont care about this for Element call -> Element X
        // return Pending until the user has confirmed the open id permission with widget.aquire_openid(),
        // then call get_openid again with the Pending oneshot channel as an optional prop so that the receiver in Pending
        // can be resolved as soon as the token is fetched from the server.

        let Some(user_id) = self.room.client.user_id() else {
            return OpenIDState::Resolved(Err(WidgetError("No user ID available".to_owned())));
        };

        let request = request_openid_token::v3::Request::new(user_id.to_owned());
        match self.room.client.send(request, None).await {
            Err(e) => OpenIDState::Resolved(Err(WidgetError(e.to_string()))),
            Ok(res) => OpenIDState::Resolved(Ok(openid::Response {
                id: req.id,
                token: res.access_token,
                expires_in_seconds: res.expires_in.as_secs() as usize,
                server: res.matrix_server_name.to_string(),
                kind: res.token_type.to_string(),
            })),
        }
    }
}

impl<W: Widget> Drop for Driver<W> {
    fn drop(&mut self) {
        if let Some(handle) = &self.add_event_handler_handle {
            self.room.client().remove_event_handler(handle.clone());
        }
    }
}

#[derive(Debug)]
pub struct EventReader {
    room: Joined,
    filter: Vec<Filter>,
}

#[async_trait]
impl handler::EventReader for EventReader {
    fn get_filter(&self) -> &Vec<Filter> {
        &self.filter
    }
    async fn read(&self, req: ReadEventRequest) -> Result<ReadEventResponse> {
        let options = {
            let mut o = MessagesOptions::backward();
            o.limit = req.limit.into();
            o.filter = {
                let mut f = RoomEventFilter::default();
                f.types = Some(vec![req.message_type]);
                f
            };
            o
        };

        let messages = self.room.messages(options).await?;

        let state_events: Vec<serde_json::Result<MatrixEvent>> =
            messages.state.iter().map(|s| s.deserialize_as()).collect();
        let mut timeline_events: Vec<serde_json::Result<MatrixEvent>> =
            messages.chunk.iter().map(|msg| msg.event.deserialize_as()).collect();
        let mut all_messages = state_events;
        all_messages.append(&mut timeline_events);

        {
            let failed_messages: Vec<&serde_json::Result<MatrixEvent>> =
                all_messages.iter().filter(|res| res.is_err()).collect();
            if failed_messages.len() > 0 {
                eprintln!("There were {} failed messages while trying to format them to send them to a widget.", failed_messages.len());
            }
        }
        let allowed_messages: Vec<MatrixEvent> = all_messages
            .into_iter()
            .filter_map(|res| res.ok())
            .filter(|m| {
                let filter_fn = |f: &Filter| f.allow_event(&m.event_type, &m.state_key, &m.content);
                self.filter.iter().any(filter_fn)
            })
            .collect();
        Ok(ReadEventResponse { events: allowed_messages })
    }
}
#[derive(Debug)]
pub struct EventSender {
    room: Joined,
    filter: Vec<Filter>,
}

#[async_trait]
impl handler::EventSender for EventSender {
    fn get_filter(&self) -> &Vec<Filter> {
        &self.filter
    }
    async fn send(&self, req: SendEventRequest) -> Result<SendEventResponse> {
        let filter_fn = |f: &Filter| f.allow_event(&req.message_type, &req.state_key, &req.content);

        if self.filter.iter().any(filter_fn) {
            match req.state_key {
                Some(state_key) => {
                    match self
                        .room
                        .send_state_event_raw(req.content, &req.message_type, &state_key)
                        .await
                    {
                        Ok(send_res) => Ok(SendEventResponse {
                            room_id: self.room.room_id().to_string(),
                            event_id: send_res.event_id.to_string(),
                        }),
                        Err(err) => {
                            Err(WidgetError(format!("Could not send event with error: {}", err)))
                        }
                    }
                }
                None => match self.room.send_raw(req.content, &req.message_type, None).await {
                    Ok(send_res) => Ok(SendEventResponse {
                        room_id: self.room.room_id().to_string(),
                        event_id: send_res.event_id.to_string(),
                    }),
                    Err(err) => {
                        Err(WidgetError(format!("Could not send event with error: {}", err)))
                    }
                },
            }
        } else {
            Err(WidgetError(format!(
                "No capability to send event of type {} with state key {} (for room events the state key is undefined if no state key is shown the state key is \"\")",
                req.message_type, req.state_key.unwrap_or("undefined".to_string())
            )))
        }
    }
}
impl<W: Widget> Driver<W> {
    fn build_event_listener(
        &mut self,
        filter: &Vec<Filter>,
    ) -> Option<mpsc::UnboundedReceiver<MatrixEvent>> {
        let (tx, rx) = mpsc::unbounded_channel::<MatrixEvent>();
        let filter = filter.clone();
        if !filter.is_empty() {
            let callback = move |ev: Raw<AnySyncTimelineEvent>| {
                let filter = filter.clone();
                let tx = tx.clone();
                async move {
                    match ev.deserialize_as::<MatrixEvent>() {
                        Ok(m_ev) => {
                            if (&filter).clone().iter().any(|f| {
                                f.allow_event(&m_ev.event_type, &m_ev.state_key, &m_ev.content)
                            }) {
                                let _= tx.send(m_ev).map_err(|err|eprintln!("Could not send sync matrix message to another thread: {err}"));
                            }
                        }
                        Err(err) => {
                            eprintln!("Could not parse AnySyncTimelineEvent as crate::widget_api::MatrixEvent: {err}");
                        }
                    }
                }
            };
            self.add_event_handler_handle = Some(self.room.add_event_handler(callback));
            return Some(rx);
        }

        None
    }
}
