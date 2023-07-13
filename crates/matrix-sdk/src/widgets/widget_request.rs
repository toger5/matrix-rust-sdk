use http::response;
use serde_json::Value;

pub enum WidgetApiDirection {
    ToWidget,
    FromWidget,
}
pub impl WidgetApiDirection {
    fn as_str(&self) -> &'static str {
        match self {
            WidgetApiDirection::ToWidget => "toWidget",
            WidgetApiDirection::FromWidget => "fromWidget",
        }
    }
}
pub enum WidgetApiToWidgetAction {
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
pub impl WidgetApiToWidgetAction {
    fn as_str(&self) -> &'static str {
        match self {
            WidgetApiToWidgetAction::SupportedApiVersions => "supported_api_versions",
            WidgetApiToWidgetAction::Capabilities => "capabilities",
            WidgetApiToWidgetAction::NotifyCapabilities => "notify_capabilities",
            WidgetApiToWidgetAction::TakeScreenshot => "screenshot",
            WidgetApiToWidgetAction::UpdateVisibility => "visibility",
            WidgetApiToWidgetAction::OpenIDCredentials => "openid_credentials",
            WidgetApiToWidgetAction::WidgetConfig => "widget_config",
            WidgetApiToWidgetAction::CloseModalWidget => "close_modal",
            WidgetApiToWidgetAction::ButtonClicked => "button_clicked",
            WidgetApiToWidgetAction::SendEvent => "send_event",
            WidgetApiToWidgetAction::SendToDevice => "send_to_device",
            WidgetApiToWidgetAction::UpdateTurnServers => "update_turn_servers",
        }
    }
}

pub enum WidgetApiFromWidgetAction {
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
pub impl WidgetApiFromWidgetAction {
    fn as_str(&self) -> &'static str {
        match self {
            WidgetApiFromWidgetAction::SupportedApiVersions => "supported_api_versions",
            WidgetApiFromWidgetAction::ContentLoaded => "content_loaded",
            WidgetApiFromWidgetAction::SendSticker => "m.sticker",
            WidgetApiFromWidgetAction::UpdateAlwaysOnScreen => "set_always_on_screen",
            WidgetApiFromWidgetAction::GetOpenIDCredentials => "get_openid",
            WidgetApiFromWidgetAction::CloseModalWidget => "close_modal",
            WidgetApiFromWidgetAction::OpenModalWidget => "open_modal",
            WidgetApiFromWidgetAction::SetModalButtonEnabled => "set_button_enabled",
            WidgetApiFromWidgetAction::SendEvent => "send_event",
            WidgetApiFromWidgetAction::SendToDevice => "send_to_device",
            WidgetApiFromWidgetAction::WatchTurnServers => "watch_turn_servers",
            WidgetApiFromWidgetAction::UnwatchTurnServers => "unwatch_turn_servers",
            WidgetApiFromWidgetAction::MSC2876ReadEvents => "org.matrix.msc2876.read_events",
            WidgetApiFromWidgetAction::MSC2931Navigate => "org.matrix.msc2931.navigate",
            WidgetApiFromWidgetAction::MSC2974RenegotiateCapabilities => {
                "org.matrix.msc2974.request_capabilities"
            }
            WidgetApiFromWidgetAction::MSC3869ReadRelations => "org.matrix.msc3869.read_relations",
            WidgetApiFromWidgetAction::MSC3973UserDirectorySearch => {
                "org.matrix.msc3973.user_directory_search"
            }
        }
    }
}

pub struct WidgetApiRequest {
    api: WidgetApiDirection,
    request_id: String,
    action: WidgetApiFromWidgetAction + WidgetApiToWidgetAction,
    widget_id: String,
    data: serde_json::Value,
}

pub impl WidgetApiRequest {
    fn from(request: Value) -> Self {
        request
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

pub struct WidgetApiResponse {
    request: WidgetApiRequest,
    response: serde_json::Value,
}
pub impl WidgetApiResponse {
    fn from(response: Value) -> Self {
        WidgetApiResponse {
            request: WidgetApiRequest {
                api: response.into(),
                request_id: (),
                action: (),
                widget_id: (),
                data: (),
            },
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
