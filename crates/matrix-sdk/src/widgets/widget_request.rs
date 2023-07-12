enum WidgetApiDirection {
    ToWidget = "toWidget",
    FromWidget = "fromWidget",
}
enum WidgetApiToWidgetAction {
    SupportedApiVersions = "supported_api_versions",
    Capabilities = "capabilities",
    NotifyCapabilities = "notify_capabilities",
    TakeScreenshot = "screenshot",
    UpdateVisibility = "visibility",
    OpenIDCredentials = "openid_credentials",
    WidgetConfig = "widget_config",
    CloseModalWidget = "close_modal",
    ButtonClicked = "button_clicked",
    SendEvent = "send_event",
    SendToDevice = "send_to_device",
    UpdateTurnServers = "update_turn_servers",
}

enum WidgetApiFromWidgetAction {
    SupportedApiVersions = "supported_api_versions",
    ContentLoaded = "content_loaded",
    SendSticker = "m.sticker",
    UpdateAlwaysOnScreen = "set_always_on_screen",
    GetOpenIDCredentials = "get_openid",
    CloseModalWidget = "close_modal",
    OpenModalWidget = "open_modal",
    SetModalButtonEnabled = "set_button_enabled",
    SendEvent = "send_event",
    SendToDevice = "send_to_device",
    WatchTurnServers = "watch_turn_servers",
    UnwatchTurnServers = "unwatch_turn_servers",

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC2876ReadEvents = "org.matrix.msc2876.read_events",

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC2931Navigate = "org.matrix.msc2931.navigate",

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC2974RenegotiateCapabilities = "org.matrix.msc2974.request_capabilities",

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC3869ReadRelations = "org.matrix.msc3869.read_relations",

    /**
     * @deprecated It is not recommended to rely on this existing - it can be removed without notice.
     */
    MSC3973UserDirectorySearch = "org.matrix.msc3973.user_directory_search",
}

struct WidgetApiRequest {
    api: WidgetApiDirection,
    requestId: String,
    action: WidgetApiFromWidgetAction + WidgetApiToWidgetAction,
    widgetId: String,
    data: serde_json::Value,
}
