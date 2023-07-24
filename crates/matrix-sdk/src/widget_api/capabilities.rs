use ruma::events::{room::message::MessageType, TimelineEventType};
use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

struct EventFilter {
    event_type: String,
    msgtype: Option<String>,
}
struct StateEventFilter {
    event_type: String,
    state_key: Option<String>,
}
// impl EventFilter{
//     fn check_event(ev: MatrixEvent) -> bool{

//     }
// }

impl Serialize for EventFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut string = format!("{}", self.event_type);
        if let Some(msgtype) = self.msgtype {
            string = format!("{}#{}", string, msgtype);
        }
        serializer.serialize_str(&string)
    }
}
impl Serialize for StateEventFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut string = format!(":{}", self.event_type);
        if let Some(state_key) = self.state_key {
            string = format!("{}#{}", string, state_key);
        }
        serializer.serialize_str(&string)
    }
}

impl<'de> Deserialize<'de> for StateEventFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let split: Vec<&str> = String::deserialize(deserializer)?.split("#").collect();
        let mut ev_type = split[0].to_owned();
        let mut state_key: Option<String> = None;
        if split.len() > 1 {
            state_key = Some(split[1].to_owned())
        }
        Ok(StateEventFilter { event_type: ev_type, state_key: state_key })
    }
}
impl<'de> Deserialize<'de> for EventFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let split: Vec<&str> = String::deserialize(deserializer)?.split("#").collect();
        let mut ev_type = split[0].to_owned();
        let mut msgtype: Option<String> = None;
        if split.len() > 1 {
            msgtype = Some(split[1].to_owned())
        }
        Ok(EventFilter { event_type: ev_type, msgtype: msgtype })
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Capabilities {
    navigate: bool,
    screenshot: bool,

    // room events
    send_room_event: Option<Vec<EventFilter>>,
    receive_room_event: Option<Vec<EventFilter>>,
    // state events
    send_state_event: Option<Vec<EventFilter>>,
    receive_state_event: Option<Vec<EventFilter>>,

    always_on_screen: bool, // "m.always_on_screen",

    requires_client: bool,
}

impl Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut capability_list = serializer.serialize_seq(None)?;
        if self.navigate {
            capability_list.serialize_element("org.matrix.msc2931.navigate")?;
        }
        if self.screenshot {
            capability_list.serialize_element("m.capability.screenshot")?;
        }
        if self.always_on_screen {
            capability_list.serialize_element("m.always_on_screen")?;
        }
        if self.always_on_screen {
            capability_list.serialize_element("m.always_on_screen")?;
        }
        if self.requires_client {
            capability_list.serialize_element("io.element.requires_client")?;
        }

        if let Some(ev_filter) = &self.send_room_event {
            for event_type in ev_filter {
                capability_list.serialize_element(&format!(
                    "org.matrix.msc2762.m.send.event{}",
                    serde_json::to_string(&event_type).unwrap()
                ))?;
            }
        }
        if let Some(ev_filter) = &self.receive_room_event {
            for event_type in ev_filter {
                capability_list.serialize_element(&format!(
                    "org.matrix.msc2762.m.receive.event{}",
                    serde_json::to_string(&event_type).unwrap()
                ))?;
            }
        }
        if let Some(ev_filter) = &self.send_state_event {
            for event_type in ev_filter {
                capability_list.serialize_element(&format!(
                    "org.matrix.msc2762.m.send.state_event{}",
                    serde_json::to_string(&event_type).unwrap()
                ))?;
            }
        }
        if let Some(ev_filter) = &self.receive_state_event {
            for event_type in ev_filter {
                capability_list.serialize_element(&format!(
                    "org.matrix.msc2762.m.receive.state_event{}",
                    serde_json::to_string(&event_type).unwrap()
                ))?;
            }
        }
        // my_string_vec.serialize(serializer);
        capability_list.end()
    }
}
impl<'de> Deserialize<'de> for Options {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let capability_list = Vec::<String>::deserialize(deserializer)?;
        println!("Capability list: {:?}", capability_list);
        // let s: &str = Deserialize::deserialize(deserializer)?;
        // let options: Vec<String> = serde_json::from_str(s).unwrap();
        let mut options = Options {
            navigate: None,
            screenshot: None,
            always_on_screen: None, // "m.always_on_screen",
            requires_client: None,
            // room events
            send_room_event: None,
            receive_room_event: None,
            // state events
            send_state_event: None,
            receive_state_event: None,
        };
        let send_room_event: Vec<EventFilter> = [];
        let receive_room_event: Vec<EventFilter> = [];
        let send_state_event: Vec<EventFilter> = [];
        let receive_state_event: Vec<EventFilter> = [];
        for capability in capability_list {
            if capability == "org.matrix.msc2931.navigate" {
                options.navigate = Some(());
            }
            if capability == "m.capability.screenshot" {
                options.screenshot = Some(());
            }
            if capability == "m.always_on_screen" {
                options.always_on_screen = Some(());
            }
            if capability == "io.element.requires_client" {
                options.requires_client = Some(());
            }
            if capability.starts_with("org.matrix.msc2762.m.send.event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    send_room_event.push(serde_json::from_str(cap_split[1]));
                }
            }
            if capability.starts_with("org.matrix.msc2762.m.receive.event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    receive_room_event.push(serde_json::from_str(cap_split[1]));
                }
            }
            if capability.starts_with("org.matrix.msc2762.m.send.state_event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    send_state_event.push(serde_json::from_str(cap_split[1]));
                }
            }
            if capability.starts_with("org.matrix.msc2762.m.receive.state_event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    receive_state_event.push(serde_json::from_str(cap_split[1]));
                }
            }
        }
        Ok(options)
    }
}

/// A wrapper for the matrix client that only exposes what is available through the capabilities.
pub struct ClientCapabilities {
    pub navigate: Option<Box<dyn Fn(Url) + Send + Sync + 'static>>,
}

impl<'t> From<&'t Capabilities> for Options {
    fn from(capabilities: &'t Capabilities) -> Self {
        Self { navigate: capabilities.navigate.is_some(), ..Default::default() }
    }
}
