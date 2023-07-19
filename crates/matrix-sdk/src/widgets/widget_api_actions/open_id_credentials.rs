use serde::{Serialize, Deserialize};

use crate::widgets::widget_message::WidgetActionBody;

use super::WidgetMessageEmptyData;

// MSC1960

#[derive(Serialize, Deserialize, Debug)]
pub enum OpenIdState{
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "request")]
    PendingUserConfirmation

}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetOpenIdResponse{
    state: OpenIdState, //OpenIDRequestState;
    original_request_id: String,
    access_token: Option<String>,
    expires_in: Option<i32>,
    matrix_server_name: Option<String>,
    token_type: Option<String>,
}

pub type FromWidgetGetOpenIdBody = WidgetActionBody<WidgetMessageEmptyData, OpenIdCredentialsResponse>;
pub type ToWidgetOpenIdCredentialsBody = WidgetActionBody<WidgetMessageEmptyData, OpenIdCredentialsResponse>;
