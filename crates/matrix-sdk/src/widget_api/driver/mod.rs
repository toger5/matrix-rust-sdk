use self::widget::Widget;
use super::{
    handler::{self, Capabilities, OpenIDState, Outgoing, Result},
    messages::{capabilities::Options, openid},
    Error,
};
use crate::room::Joined;
use async_trait::async_trait;
use ruma::events::AnySyncTimelineEvent;
use ruma::serde::Raw;
use tokio::sync::{mpsc, Mutex};

pub mod widget;

#[derive(Debug)]
pub struct Driver<W: Widget> {
    pub matrix_room: Joined,
    pub widget: W,
    add_event_handler_handle: Option<EventHandlerHandle>,
}
#[async_trait(?Send)]
impl<W: Widget> handler::Driver for Driver<W> {
    // was initially defined as async by Daniel but for simplicity Timo made it sync for now
    async fn send(&self, message: Outgoing) -> Result<()> {
        // let message_str = serde_json::to_string(&message)?;
        self.widget.send_widget_message("TODO get message string from outgoing");
        Result::Ok(())
    }
    // was initially defined as async by Daniel but for simplicity Timo made it sync for now
    fn initialise(&self, options: Options) -> Result<handler::Capabilities> {
        let mut capabilities =
            handler::Capabilities { options: options.clone(), ..handler::Capabilities::default() };
        // capabilities.send_room_event = self.build_send_room_event(&options);

        // capabilities.add_matrix_room_event_listener =
        //     self.build_add_matrix_room_event_listener(&options);
        let room_event_sender_filter = options.send_room_event.as_ref().unwrap_or(&vec![]).to_vec();
        if room_event_sender_filter.len() > 0 {
            capabilities.room_event_sender = Some(Box::new(RoomEventSender {
                room: self.matrix_room.clone(),
                filter: room_event_sender_filter,
            }))
        }
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
    // fn build_send_room_event(
    //     &self,
    //     options: &Options,
    // ) -> Option<dyn handler::EventSender + 'static> {
    //     let mut event_sender = None;
    //     let send_event_filter = options.send_room_event.as_ref().unwrap_or(&vec![]);
    //     let m_room = self.matrix_room;
    //     if send_event_filter.len() > 0 {
    //         let (tx, rx) = mpsc::channel::<MatrixEvent>(1);
    //         tokio::spawn(async move {
    //             while let Some(matrix_event) = rx.recv().await {
    //                 if send_event_filter.iter().any(|f| f.allow_event(&matrix_event)) {
    //                     m_room.send_raw(matrix_event.content, &matrix_event.event_type, None);
    //                 } else {
    //                 }
    //             }
    //         });

    //         event_sender = Some(tx);
    //     }
    //     event_sender
    // }

    fn build_add_matrix_room_event_listener(
        &self,
        receive_filters: &Option<Vec<EventFilter>>,
    ) -> Option<mpsc::Receiver<MatrixEvent>> {
        let mut room_event_listener = None;
        let room_id = self.matrix_room.room_id().to_string();
        let receive_event_filter = receive_filters.as_ref().unwrap_or(&vec![]).to_vec();

        if receive_event_filter.len() > 0 {
            let (tx, rx) = mpsc::channel::<MatrixEvent>(1);
            room_event_listener = Some(rx);
            let filter = receive_event_filter.clone();
            let callback = {
                let t_mutex = Mutex::new(Some(tx));
                move |ev: Raw<AnySyncTimelineEvent>| async {
                    match ev.deserialize_as::<MatrixEvent>() {
                        Ok(m_ev) => {
                            if vec![EventFilter { event_type: "todo!()".to_owned(), msgtype: None }]
                                .clone()
                                .iter()
                                .any(|f| {
                                    f.allow_event(&m_ev.event_type, &m_ev.state_key, &m_ev.content)
                                })
                            {
                                if let Some(tx) = (&t_mutex).lock().await.take() {
                                    let _res = tx.send(m_ev).await;
                                }
                            }
                        }
                        Err(error) => {}
                    }
                }
            };
            self.matrix_room.add_event_handler(callback);
        }
        // let mut add_matrix_room_event_listener_capability = None;
        // let room_id = self.matrix_room.room_id().to_string();
        // let receive_event_filter = options.receive_room_event.as_ref().unwrap_or(&vec![]);
        // if receive_event_filter.len() > 0 {
        //     let reveive_event_closure: Box<dyn Fn(OnEventCallback)> =
        //         Box::new(|on_event: OnEventCallback| {
        //             let handle =
        //                 self.matrix_room.add_event_handler(|ev: SyncRoomMessageEvent| async move {
        //                     // if receive_event_filter.iter().any(|filter|filter.allow_event(m)){
        //                     // on_event(m)
        //                     // }
        //                     //Do the logic to filter with the filters
        //                     on_event(MatrixEvent {
        //                         event_type: ev.event_type().to_string(),
        //                         sender: ev.sender().to_string(),
        //                         event_id: ev.event_id().to_string(),
        //                         room_id,
        //                         state_key: None,
        //                         origin_server_ts: ev.origin_server_ts().get() as u32,
        //                         content: json!({}), /*TODO get content */
        //                         unsigned: Unsigned { age: 0 }, /*TODO get unsigned */
        //                     })
        //                 });
        //             self.add_event_handler_handle = Some(handle);
        //         });
        //     add_matrix_room_event_listener_capability = Some(reveive_event_closure);
        // }
        // add_matrix_room_event_listener_capability
        room_event_listener
    }
}
