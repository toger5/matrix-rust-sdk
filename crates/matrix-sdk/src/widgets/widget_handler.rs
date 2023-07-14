use std::sync::Arc;

use ruma::events::room::message::RoomMessageEventContent;

use super::{
    widget_client_driver::WidgetClientDriver,
    widget_matrix_driver::WidgetMatrixDriver,
    widget_driver::WidgetDriver,
    widget_message::{FromWidgetAction, WidgetMessageDirection, WidgetMessageRequest},
};

pub trait WidgetMessageHandler {
    fn handle_content_loaded(&self, request: WidgetMessageRequest);
    fn handle_read_events(&self, request: WidgetMessageRequest);
    fn handle_unimplemented_request(&self, request: WidgetMessageRequest);
}
impl<CD: WidgetClientDriver> WidgetMessageHandler for WidgetDriver<CD> {
    // pub async fn handle(&self, message: &str){
    //     let request = WidgetMessageRequest{
    //         api: WidgetMessageDirection::FromWidget,
    //         request_id: String::from("request_id1234"),
    //         action: FromWidgetAction::ContentLoaded,
    //         widget_id: String::from("widget_id1234"),
    //         data: serde_json::json!({"data":"10"}),
    //     };
    //     // here we want to have a big match
    //     match request.action {
    //         FromWidgetAction::ContentLoaded => self.handle_content_loaded(request),
    //         FromWidgetAction::MSC2876ReadEvents => self.handle_read_events(request),
    //         default => self.handle_unimplemented_request(request)
    //     }
    //     let content =
    //         RoomMessageEventContent::text_plain(message.to_owned() + &String::from("normal send"));
    //     let r = self.room.clone().unwrap();
    //     let _ = r.send_raw(serde_json::json!({"body":"test"}), "customWidgetType", None).await;
    //     let _ = r.send(content, None).await;
    // }

    // private
    fn handle_content_loaded(&self, request: WidgetMessageRequest) {}
    fn handle_read_events(&self, request: WidgetMessageRequest) {}
    fn handle_unimplemented_request(&self, request: WidgetMessageRequest) {
        // let delegate = Arc::clone(&self.toWidgetDelegate);
        // if let Some(delegate) = delegate.read().unwrap().as_ref() {
        //     //delegate.to_widget(request.)
        // }
    }
}
