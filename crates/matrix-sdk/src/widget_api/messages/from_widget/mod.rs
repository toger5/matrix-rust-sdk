pub mod message_types;
pub mod reply;

use serde::{Deserialize, Serialize};

pub use self::{
    reply::Reply,
};
pub use super::super::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request<ReqBody, ResBody> {
    pub content: ReqBody,
    #[serde(skip_serializing)]
    reply: Reply<ReqBody, ResBody>,
}

impl<C, R> Request<C, R> {
    pub fn reply(self, response: R) -> Result<(), Error> {
        self.reply.reply(response).map_err(|_| Error::WidgetDied)
    }
}
