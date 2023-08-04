use self::widget::Widget;
use super::{
    handler::{self, Capabilities, OpenIDState, Outgoing, Result},
    messages::{capabilities::Options, openid},
    Error,
};
use crate::room::Joined;
use async_trait::async_trait;

pub mod widget;

#[derive(Debug)]
pub struct Driver<W: Widget> {
    pub matrix_room: Joined,
    pub widget: W,
}
#[async_trait(?Send)]
impl<W: Widget> handler::Driver for Driver<W> {
    fn initialise(&self, options: Options) -> Result<Capabilities> {
        unimplemented!()
    }
    async fn send(&self, message: Outgoing) -> Result<()> {
        unimplemented!()
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
