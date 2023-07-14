use http::response;
use serde_json::Value;

pub enum WidgetMessageDirection {
    ToWidget,
    FromWidget,
}

pub enum WidgetAction {
    FromWidget(FromWidgetAction),
    ToWidget(ToWidgetAction),
}
impl WidgetAction {
    pub fn as_str(&self){

    }
}
impl WidgetMessageDirection {
    fn as_str(&self) -> &'static str {
        match self {
            WidgetMessageDirection::ToWidget => "toWidget",
            WidgetMessageDirection::FromWidget => "fromWidget",
        }
    }
}
pub enum ToWidgetAction {
    SupportedApiVersions,
    Capabilities,
    NotifyCapabilities,
    TakeScreenshot,
    UpdateVisibility,
    OpenIDCredentials,
    WidgetConfig,
    CloseModalWidget,
    ButtonClicked,
    SendEvent,
    SendToDevice,
    UpdateTurnServers,
}
impl ToWidgetAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToWidgetAction::SupportedApiVersions => "supported_api_versions",
            ToWidgetAction::Capabilities => "capabilities",
            ToWidgetAction::NotifyCapabilities => "notify_capabilities",
            ToWidgetAction::TakeScreenshot => "screenshot",
            ToWidgetAction::UpdateVisibility => "visibility",
            ToWidgetAction::OpenIDCredentials => "openid_credentials",
            ToWidgetAction::WidgetConfig => "widget_config",
            ToWidgetAction::CloseModalWidget => "close_modal",
            ToWidgetAction::ButtonClicked => "button_clicked",
            ToWidgetAction::SendEvent => "send_event",
            ToWidgetAction::SendToDevice => "send_to_device",
            ToWidgetAction::UpdateTurnServers => "update_turn_servers",
        }
    }
}

pub enum FromWidgetAction {
    SupportedApiVersions,
    ContentLoaded,
    SendSticker,
    UpdateAlwaysOnScreen,
    GetOpenIDCredentials,
    CloseModalWidget,
    OpenModalWidget,
    SetModalButtonEnabled,
    SendEvent,
    SendToDevice,
    WatchTurnServers,
    UnwatchTurnServers,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC2876ReadEvents,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC2931Navigate,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC2974RenegotiateCapabilities,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC3869ReadRelations,

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC3973UserDirectorySearch,
}
impl FromWidgetAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            FromWidgetAction::SupportedApiVersions => "supported_api_versions",
            FromWidgetAction::ContentLoaded => "content_loaded",
            FromWidgetAction::SendSticker => "m.sticker",
            FromWidgetAction::UpdateAlwaysOnScreen => "set_always_on_screen",
            FromWidgetAction::GetOpenIDCredentials => "get_openid",
            FromWidgetAction::CloseModalWidget => "close_modal",
            FromWidgetAction::OpenModalWidget => "open_modal",
            FromWidgetAction::SetModalButtonEnabled => "set_button_enabled",
            FromWidgetAction::SendEvent => "send_event",
            FromWidgetAction::SendToDevice => "send_to_device",
            FromWidgetAction::WatchTurnServers => "watch_turn_servers",
            FromWidgetAction::UnwatchTurnServers => "unwatch_turn_servers",
            FromWidgetAction::MSC2876ReadEvents => "org.matrix.msc2876.read_events",
            FromWidgetAction::MSC2931Navigate => "org.matrix.msc2931.navigate",
            FromWidgetAction::MSC2974RenegotiateCapabilities => {
                "org.matrix.msc2974.request_capabilities"
            }
            FromWidgetAction::MSC3869ReadRelations => "org.matrix.msc3869.read_relations",
            FromWidgetAction::MSC3973UserDirectorySearch => {
                "org.matrix.msc3973.user_directory_search"
            }
        }
    }
}


pub enum WidgetMessage {
    Request(WidgetMessageRequest),
    Response(WidgetMessageResponse)
}
pub struct WidgetMessageRequest {
    pub api: WidgetMessageDirection,
    pub request_id: String,
    pub action: WidgetAction,
    pub widget_id: String,
    pub data: serde_json::Value,
}

impl WidgetMessageRequest {
    fn from(request: Value) -> Option<Self> {
        None
    }
    fn as_value(&self) -> serde_json::Value {
        serde_json::json!({
            "api": self.api.as_str(),
            "request_id": &self.request_id,
            "action": &self.action.as_str(),
            "widget_id": &self.widget_id
        })
    }
}

pub struct WidgetMessageResponse {
    request: WidgetMessageRequest,
    response: serde_json::Value,
}
pub impl WidgetMessageResponse {
    fn from(response: Value) -> Self {
        WidgetMessageResponse {
            request: WidgetMessageRequest {
                api: "response.get(request)",
                request_id: (),
                action: (),
                widget_id: (),
                data: (),
            },
            response: serde_json::json!({"data":"example data TODO implement serialization"})
        }
    }
    fn as_value(&self) -> serde_json::Value {
        serde_json::json!({
            "api":self.api.as_str(),
            "request_id": self.request_id,
            "action": self.action.as_str(),
            "widget_id": self.
        })
    }
}
