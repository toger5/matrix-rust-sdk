use super::widget_api_actions::{
    capabilities::CapabilitiesResponse, open_id_credentials::OpenIdCredentialsResponse,
    supported_api_versions::SupportedApiVersionsResponse, WidgetError, WidgetMessageEmptyData, send_to_device::SendToDeviceBody,
};
use serde::{Deserialize, Serialize};

// We are modeling this json Structure
// {
//     "request_id": String,
//     "widget_id": String,

//     "api": "toWidget" | "fromWidget",
//     "action": String

//     "data":Option<Data>, // request
//     "response": {Data} | {message: "errormessage"}
// }

// into

// enum WidgetMessage {
//     FromWidget(
//         header: WidgetMessageHeader {
//             request_id: String
//             widget_id: String
//         }
//         body: enum FromWidgetAction {
//             SupportedApiVersions(
//                 request: Option{Value},
//                 response: Option{ enum {
//                     Error(
//                         message: String
//                     ),
//                     Response(Value)
//                     }
//                 }
//             ),
//             ContentLoaded(WidgetAction),
//             ...
//         }
//     ),
//     ToWidget(
//         header: WidgetMessageHeader {
//             request_id: String
//             widget_id: String
//         }
//         body: enum ToWidgetAction {
//             SupportedApiVersions(
//                 request: Option{Value},
//                 response: Option{enum {
//                         Error(
//                             message: String
//                         ),
//                         Response(Value)
//                     }
//                 }
//             ),
//             Capabilities(WidgetAction),
//             NotifyCapabilities(WidgetAction),
//             TakeScreenshot(WidgetAction),
//             ...
//         }
//     )
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "api")]
// pub enum WidgetMessage {
//     #[serde(rename = "fromWidget")]
//     FromWidget {
//         #[serde(flatten)]
//         header: WidgetMessageHeader,
//         #[serde(flatten)]
//         action: FromWidgetAction,
//     },
//     #[serde(rename = "toWidget")]
//     ToWidget {
//         #[serde(flatten)]
//         header: WidgetMessageHeader,
//         #[serde(flatten)]
//         action: ToWidgetAction,
//     },
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "api")]
pub enum WidgetMessage {
    #[serde(rename = "fromWidget")]
    FromWidget(FromWidgetAction),
    #[serde(rename = "toWidget")]
    ToWidget(ToWidgetAction),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WidgetActionBody<Req, Res> {
    #[serde(rename = "request_id")]
    pub request_id: String,
    #[serde(rename = "widget_id")]
    pub widget_id: String,
    #[serde(rename = "data")]
    request: Option<Req>,

    response: Option<Response<Res>>,
}
impl<Req, Res> WidgetActionBody<Req, Res> {
    pub fn is_response(&self) -> bool {
        self.response.is_some()
    }
}

// We cannot use Result here because it does not Serialize the way we need it to
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<Res> {
    Error(WidgetError),
    Response(Res),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum FromWidgetAction {
    #[serde(rename = "supported_api_versions")]
    SupportedApiVersions,
    #[serde(rename = "content_loaded")]
    ContentLoaded,
    #[serde(rename = "m.sticker")]
    SendSticker,
    #[serde(rename = "set_always_on_screen")]
    UpdateAlwaysOnScreen,
    #[serde(rename = "get_openid")]
    GetOpenIDCredentials,
    #[serde(rename = "close_modal")]
    CloseModalWidget,
    #[serde(rename = "open_modal")]
    OpenModalWidget,
    #[serde(rename = "set_button_enabled")]
    SetModalButtonEnabled,
    #[serde(rename = "send_event")]
    SendEvent,
    #[serde(rename = "send_to_device")]
    SendToDevice(SendToDeviceBody),
    #[serde(rename = "watch_turn_servers")]
    WatchTurnServers,
    #[serde(rename = "unwatch_turn_servers")]
    UnwatchTurnServers,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    #[serde(rename = "org.matrix.msc2876.read_events")]
    MSC2876ReadEvents,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    #[serde(rename = "org.matrix.msc2931.navigate")]
    MSC2931Navigate,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    #[serde(rename = "org.matrix.msc2974.request_capabilities")]
    MSC2974RenegotiateCapabilities,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    #[serde(rename = "org.matrix.msc3869.read_relations")]
    MSC3869ReadRelations,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    #[serde(rename = "org.matrix.msc3973.user_directory_search")]
    MSC3973UserDirectorySearch,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
pub enum ToWidgetAction {
    #[serde(rename = "supported_api_versions")]
    SupportedApiVersions(WidgetActionBody<WidgetMessageEmptyData, SupportedApiVersionsResponse>),
    #[serde(rename = "capabilities")]
    Capabilities(WidgetActionBody<WidgetMessageEmptyData, CapabilitiesResponse>),
    #[serde(rename = "notify_capabilities")]
    NotifyCapabilities,
    #[serde(rename = "screenshot")]
    TakeScreenshot,
    #[serde(rename = "visibility")]
    UpdateVisibility,
    #[serde(rename = "openid_credentials")]
    OpenIDCredentials(WidgetActionBody<WidgetMessageEmptyData, OpenIdCredentialsResponse>),
    #[serde(rename = "widget_config")]
    WidgetConfig,
    #[serde(rename = "close_modal")]
    CloseModalWidget,
    #[serde(rename = "button_clicked")]
    ButtonClicked,
    #[serde(rename = "send_event")]
    SendEvent,
    #[serde(rename = "send_to_device")]
    SendToDevice,
    #[serde(rename = "update_turn_servers")]
    UpdateTurnServers,
}
