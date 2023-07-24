use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

use super::super::message::{MessageBody, Response};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reply<Req, Resp> {
    request: MessageBody<Req, Resp>,
    response: Sender<MessageBody<Req, Resp>>,
}

impl<Req, Resp> Reply<Req, Resp> {
    pub fn new(request: MessageBody<Req, Resp>, response: Sender<MessageBody<Req, Resp>>) -> Self {
        Self { request, response }
    }

    pub fn reply(self, response: Resp) -> Result<Req, Resp> {
        let message = MessageBody {
            widget_id: self.request.widget_id,
            request_id: self.request.request_id,
            request: self.request.request,
            response: Some(Response::Response(response)),
        };

        self.response.send(message).map_err(|r| {
            // Safe to unwrap here, because the `response` is always `Some()` (see above).
            let result: Result<Req, Resp> = r.response.unwrap().into();
            result.unwrap()
        })
    }
}
