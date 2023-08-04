use self::widget::Widget;
use super::{
    handler::{self, Capabilities, OpenIDState},
    messages::{
        capabilities::{EventFilter, Filter, Options},
        from_widget::{SendEventRequest, SendEventResponse},
        {openid, MatrixEvent},
    },
    {Error, Result},
};
use crate::{event_handler::EventHandlerHandle, room::Joined};
use async_trait::async_trait;
use ruma::{events::AnySyncTimelineEvent, serde::Raw};
use tokio::sync::mpsc;

pub mod widget;

#[derive(Debug)]
pub struct Driver<W: Widget> {
    pub matrix_room: Joined,
    pub widget: W,
    add_event_handler_handle: Option<EventHandlerHandle>,
}

#[async_trait(?Send)]
impl<W: Widget> handler::Driver for Driver<W> {
    async fn send(&self, message: handler::Outgoing) -> Result<()> {
        // let message_str = serde_json::to_string(&message)?;
        let _ = self.widget.send_widget_message("TODO get message string from outgoing");
        Result::Ok(())
    }
    fn initialise(&mut self, options: Options) -> Result<Capabilities> {
        let mut capabilities = Capabilities::default();

        capabilities.event_listener =
            self.build_event_listener(&options.read_room_event, &options.read_state_event);
        capabilities.event_sender =
            self.build_event_sender(&options.send_room_event, &options.send_state_event);

        Result::Ok(capabilities)
    }
    async fn get_openid(&self, req: openid::Request) -> OpenIDState {
        // TODO: make the client ask the user first.
        // We wont care about this for Element call -> Element X
        // if !self.has_open_id_user_permission() {
        //     let (rx,tx) = tokio::oneshot::channel();
        //     return OpenIDState::Pending(tx);
        //     widget.show_get_openid_dialog().await?;
        //     self.get_openid(req, Some(tx)); // get open id can be called with or without tx and either reutrns as return or sends return val over tx
        // }

        let user_id = self.matrix_room.client.user_id();
        if user_id == None {
            return OpenIDState::Resolved(Err(Error::WidgetError(
                "Failed to get an open id token from the homeserver. Because the userId is not available".to_owned()
            )));
        }
        let user_id = user_id.unwrap();

        let request =
            ruma::api::client::account::request_openid_token::v3::Request::new(user_id.to_owned());
        let res = self.matrix_room.client.send(request, None).await;

        let state = match res {
            Err(err) => Err(Error::WidgetError(
                format!(
                    "Failed to get an open id token from the homeserver. Because of Http Error: {}",
                    err.to_string()
                )
                .to_owned(),
            )),
            Ok(res) => Ok(openid::Response {
                id: req.id,
                token: res.access_token,
                expires_in_seconds: res.expires_in.as_secs() as usize,
                server: res.matrix_server_name.to_string(),
                kind: res.token_type.to_string(),
            }),
        };
        OpenIDState::Resolved(state)
    }
}
#[derive(Debug)]
pub struct EventSender {
    room: Joined,
    state_filter: Vec<EventFilter>,
    room_filter: Vec<EventFilter>,
}
#[async_trait]
impl handler::EventSender for EventSender {
    async fn send(&self, req: SendEventRequest) -> Result<SendEventResponse> {
        let filter_fn =
            |f: &EventFilter| f.allow_event(&req.message_type, &req.state_key, &req.content);

        let mut filter = self.state_filter.clone();
        filter.append(&mut self.room_filter.clone());

        if filter.iter().any(filter_fn) {
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
                        Err(err) => Err(Error::WidgetError(format!(
                            "Could not send event with error: {}",
                            err
                        ))),
                    }
                }
                None => match self.room.send_raw(req.content, &req.message_type, None).await {
                    Ok(send_res) => Ok(SendEventResponse {
                        room_id: self.room.room_id().to_string(),
                        event_id: send_res.event_id.to_string(),
                    }),
                    Err(err) => {
                        Err(Error::WidgetError(format!("Could not send event with error: {}", err)))
                    }
                },
            }
        } else {
            Err(Error::WidgetError(format!(
                "No capability to send event of type {} with state key {} (for room events the state key is undefined if no state key is shown the state key is \"\")",
                req.message_type, req.state_key.unwrap_or("undefined".to_string())
            )))
        }
    }
}
impl<W: Widget> Driver<W> {
    fn build_event_sender(
        &self,
        room_filter: &Vec<EventFilter>,
        state_filter: &Vec<EventFilter>,
    ) -> Option<Box<dyn handler::EventSender>> {
        let mut filter = state_filter.clone();
        filter.append(&mut room_filter.clone());

        if filter.len() > 0 {
            let s: Box<dyn handler::EventSender> = Box::new(EventSender {
                room: self.matrix_room.clone(),
                room_filter: room_filter.clone(),
                state_filter: state_filter.clone(),
            });
            return Some(s);
        }
        None
    }

    fn build_event_listener(
        &mut self,
        room_filter: &Vec<EventFilter>,
        state_filter: &Vec<EventFilter>,
    ) -> Option<mpsc::UnboundedReceiver<MatrixEvent>> {
        let (tx, rx) = mpsc::unbounded_channel::<MatrixEvent>();
        let mut filter = room_filter.clone();
        filter.append(&mut state_filter.clone());

        if filter.len() > 0 {
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
            self.add_event_handler_handle = Some(self.matrix_room.add_event_handler(callback));
            return Some(rx);
        }

        None
    }
}
