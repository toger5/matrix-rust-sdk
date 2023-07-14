use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]

pub enum WidgetMessageDirection {
    ToWidget,
    FromWidget,
}

#[derive(Serialize, Deserialize, Debug)]

pub enum WidgetAction {
    #[serde(rename = "fromWidget")]
    FromWidget(FromWidgetAction),
    #[serde(rename = "toWidget")]
    ToWidget(ToWidgetAction),
}

impl WidgetMessageDirection {
    fn as_str(&self) -> &'static str {
        match self {
            WidgetMessageDirection::ToWidget => "toWidget",
            WidgetMessageDirection::FromWidget => "fromWidget",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub enum ToWidgetAction {
    #[serde(rename = "supported_api_versions")]
    SupportedApiVersions,
    #[serde(rename = "capabilities")]
    Capabilities,
    #[serde(rename = "notify_capabilities")]
    NotifyCapabilities,
    #[serde(rename = "screenshot")]
    TakeScreenshot,
    #[serde(rename = "visibility")]
    UpdateVisibility,
    #[serde(rename = "openid_credentials")]
    OpenIDCredentials,
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

#[derive(Serialize, Deserialize, Debug)]

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
    SendToDevice,
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
pub enum WidgetMessage {
    Request(WidgetMessageRequest),
    Response(WidgetMessageResponse),
}

#[derive(Serialize, Deserialize, Debug)]

pub struct WidgetMessageRequest {
    #[serde(rename = "api")]
    pub api_direction: WidgetMessageDirection,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "action")]
    pub action: WidgetAction,
    #[serde(rename = "widgetId")]
    pub widget_id: String,
    #[serde(rename = "data")]
    pub data: Value,
}
// serelize tags
#[derive(Serialize, Deserialize, Debug)]
pub struct WidgetMessageResponse{
    request: WidgetMessageRequest,
    response: Value,
}


// {
//     "api_direction":"data"
//     "request_id":"data"
//     "action":"data"
//     "widget_id":"data"
//     "data":"data"
//     "response":"data"
// }

// {
//     "api_direction":"data"
//     "request_id":"data"
//     "action":"data"
//     "widget_id":"data"
//     "data":"data"
//
// }