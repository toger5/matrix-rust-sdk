use self::widget::Widget;
use super::{
    handler::{self, OpenIDState},
    messages::{
        capabilities::{EventFilter, Filter, Options},
        from_widget::{SendEventRequest, SendEventResponse},
        {openid, MatrixEvent},
    },
    {Error, Result},
};
use crate::room::Joined;
use async_trait::async_trait;

pub mod widget;

#[derive(Debug)]
pub struct Driver<W: Widget> {
    pub matrix_room: Joined,
    pub widget: W,
    add_event_handler_handle: Option<EventHandlerHandle>,
}

#[async_trait]
impl<W: Widget> handler::Driver for Driver<W> {
    // was initially defined as async by Daniel but for simplicity Timo made it sync for now
    async fn send(&self, message: Outgoing) -> Result<()> {
        // let message_str = serde_json::to_string(&message)?;
        self.widget.send_widget_message("TODO get message string from outgoing");
        Result::Ok(())
    }
    // was initially defined as async by Daniel but for simplicity Timo made it sync for now
    fn initialise(&self, options: Options) -> Result<handler::Capabilities> {
        let mut capabilities = handler::Capabilities::new(options.clone());

        let room_event_sender_filter = options.send_room_event.as_ref().unwrap_or(&vec![]).to_vec();
        if room_event_sender_filter.len() > 0 {
            capabilities.room_event_sender = Some(Box::new(RoomEventSender {
                room: self.matrix_room.clone(),
                filter: room_event_sender_filter,
            }))
        }

        capabilities.room_event_listener = self.build_room_event_listener(&options.read_room_event);

        Result::Ok(capabilities)
    }
    async fn get_openid(&self, req: openid::Request) -> OpenIDState {
        // TODO: make the client ask the user first.
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
struct RoomEventSender {
    room: Joined,
    filter: Vec<EventFilter>,
}
#[async_trait]
impl handler::EventSender for RoomEventSender {
    async fn send(&self, req: SendEventRequest) -> Result<SendEventResponse> {
        if self
            .filter
            .iter()
            .any(|f| f.allow_event(&req.message_type, &req.state_key, &req.content))
        {
            self.room.send_raw(req.content, &req.message_type, None);
            Ok(SendEventResponse { room_id: "".to_string(), event_id: "".to_string() })
        } else {
            Err(Error::WidgetError(format!(
                "No capability to send room event of type {} with key {}",
                req.message_type,
                req.state_key.unwrap_or("undefined".to_owned())
            )))
        }
    }
}
impl<W: Widget> Driver<W> {
    fn build_room_event_sender(
        &self,
        send_filter: &Option<Vec<EventFilter>>,
    ) -> Option<Box<RoomEventSender>> {
        let filter = send_filter.as_ref().unwrap_or(&vec![]).to_vec();
        let mut sender = None;
        if filter.len() > 0 {
            sender = Some(Box::new(RoomEventSender { room: self.matrix_room.clone(), filter }))
        }
        sender
    }

    fn build_room_event_listener(
        &self,
        receive_filters: &Option<Vec<EventFilter>>,
    ) -> Option<mpsc::UnboundedReceiver<MatrixEvent>> {
        let mut listener = None;
        let filter = receive_filters.as_ref().unwrap_or(&vec![]).to_vec();

        if filter.len() > 0 {
            let (tx, rx) = mpsc::unbounded_channel::<MatrixEvent>();

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

            self.matrix_room.add_event_handler(callback);
            listener = Some(rx);
        }
        listener
    }
}
