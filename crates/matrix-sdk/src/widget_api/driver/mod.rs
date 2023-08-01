use ruma::events::room::message::SyncRoomMessageEvent;
use serde_json::json;

use crate::event_handler::EventHandlerHandle;
use crate::room::Joined;

use super::capabilities::OnEventCallback;
use super::error::Result;
use super::messages::capabilities::{Filter, Options};
use super::messages::{MatrixEvent, Unsigned};
use super::Error;
use super::{capabilities::Capabilities, handler,  widget::Widget};
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
    fn build_send_room_event(
        &self,
        options: &Options,
    ) -> Option<Box<dyn Fn(MatrixEvent) -> Result<()>>> {
        let mut send_event_capability = None;
        let send_event_filter = options.send_room_event.as_ref().unwrap_or(&vec![]);
        let m_room = self.matrix_room;
        if send_event_filter.len() > 0 {
            let send_event_closure: Box<dyn Fn(MatrixEvent) -> Result<()>> =
                Box::new(move |matrix_event: MatrixEvent| -> Result<()> {
                    if send_event_filter.iter().any(|filter| filter.allow_event(&matrix_event)) {
                        m_room.send_raw(matrix_event.content, &matrix_event.event_type, None);

                        Result::<()>::Ok(())
                    } else {
                        Err(Error::WidgetError(
                            "Tried to send an event without sufficient capabilities".to_string(),
                        ))
                    }
                });
            send_event_capability = Some(send_event_closure);
        }
        send_event_capability
    }

    fn build_add_matrix_room_event_listener(
        &self,
        options: &Options,
    ) -> Option<Box<dyn Fn(OnEventCallback)>> {
        let mut add_matrix_room_event_listener_capability = None;
        let room_id = self.matrix_room.room_id().to_string();
        let receive_event_filter = options.receive_room_event.as_ref().unwrap_or(&vec![]);
        if receive_event_filter.len() > 0 {
            let reveive_event_closure: Box<dyn Fn(OnEventCallback)> =
                Box::new(|on_event: OnEventCallback| {
                    let handle = self.matrix_room.add_event_handler(
                        |ev: SyncRoomMessageEvent| async move {
                            // if receive_event_filter.iter().any(|filter|filter.allow_event(m)){
                            // on_event(m)
                            // }
                            //Do the logic to filter with the filters
                            on_event(MatrixEvent { event_type: ev.event_type().to_string(), sender: ev.sender().to_string(), event_id: ev.event_id().to_string(), room_id, state_key: None, origin_server_ts: ev.origin_server_ts().get() as u32, content: json!({}) /*TODO get content */, unsigned: Unsigned{age:0}/*TODO get unsigned */ })
                        },
                    );
                    self.add_event_handler_handle = Some(handle);
                });
            add_matrix_room_event_listener_capability = Some(reveive_event_closure);
        }
        add_matrix_room_event_listener_capability
    }
}
