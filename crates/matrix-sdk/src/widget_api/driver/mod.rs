use ruma::events::{OriginalSyncMessageLikeEvent, SyncMessageLikeEvent, AnySyncMessageLikeEvent, AnyTimelineEvent, AnySyncTimelineEvent};
use ruma::events::room::message::{SyncRoomMessageEvent, OriginalSyncRoomMessageEvent};
use ruma::serde::Raw;
use serde_json::json;
use tokio::sync::mpsc;

use crate::event_handler::{EventHandlerHandle, SyncEvent};
use crate::room::Joined;

use super::capabilities::OnEventCallback;
use super::error::Result;
use super::messages::capabilities::{EventFilter, Filter, Options};
use super::messages::{MatrixEvent, Unsigned};
use super::Error;
use super::{capabilities::Capabilities, handler, widget::Widget};
use crate::widget_api::handler::Outgoing;
pub struct Driver<W: Widget> {
    pub matrix_room: Joined,
    pub widget: W,
    add_event_handler_handle: Option<EventHandlerHandle>,
}
impl<W: Widget> handler::Driver for Driver<W> {
    // was initially defined as async by Daniel but for simplicity Timo made it sync for now
    fn send(&self, message: Outgoing) -> Result<()> {
        // let message_str = serde_json::to_string(&message)?;
        self.widget.send_widget_message("TODO get message string from outgoing");
        Result::Ok(())
    }
    // was initially defined as async by Daniel but for simplicity Timo made it sync for now
    fn initialise(&self, options: Options) -> Result<Capabilities> {
        let mut capabilities = Capabilities::default();

        capabilities.send_room_event = self.build_send_room_event(&options);

        capabilities.add_matrix_room_event_listener =
            self.build_add_matrix_room_event_listener(&options);
        Result::Ok(capabilities)
    }
}

impl<W: Widget> Driver<W> {
    fn build_send_room_event(&self, options: &Options) -> Option<mpsc::Sender<MatrixEvent>> {
        let mut event_sender = None;
        let send_event_filter = options.send_room_event.as_ref().unwrap_or(&vec![]);
        let m_room = self.matrix_room;
        if send_event_filter.len() > 0 {
            let (tx, rx) = mpsc::channel::<MatrixEvent>(1);
            tokio::spawn(async move {
                while let Some(matrix_event) = rx.recv().await {
                    if send_event_filter.iter().any(|f| f.allow_event(&matrix_event)) {
                        m_room.send_raw(matrix_event.content, &matrix_event.event_type, None);
                    } else {
                    }
                }
            });

            event_sender = Some(tx);
        }
        event_sender
    }

    fn build_add_matrix_room_event_listener(
        &self,
        receive_filters: &Option<Vec<EventFilter>>,
    ) -> Option<mpsc::Receiver<MatrixEvent>> {
        let mut room_event_listener = None;
        let room_id = self.matrix_room.room_id().to_string();
        let receive_event_filter = receive_filters.as_ref().unwrap_or(&vec![]);

        if receive_event_filter.len() > 0 {
            let (tx, rx) = mpsc::channel::<MatrixEvent>(1);
            room_event_listener = Some(rx);

            self.matrix_room.add_event_handler(|ev: Raw<AnySyncTimelineEvent>| async {
                match ev.deserialize_as::<MatrixEvent>() {
                    Ok(m_ev)=> {
                        if send_event_filter.iter().any(|f| f.allow_event(&matrix_event)) {
                            tx.send(m_ev);
                        }
                    },
                    Err(error) => {

                    }
                }
            });
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
