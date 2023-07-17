use serde::{Serialize, Deserialize};


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
pub struct OpenIdCredentialsResponse{
    state: OpenIdState, //OpenIDRequestState;
    original_request_id: String,
    access_token: Option<String>,
    expires_in: Option<i32>,
    matrix_server_name: Option<String>,
    token_type: Option<String>,
}