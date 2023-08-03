use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::MatrixEvent;

#[derive(Debug, Default, Clone)]
pub struct Options {
    pub screenshot: bool,

    // room events
    pub send_room_event: Vec<EventFilter>,
    pub read_room_event: Vec<EventFilter>,
    // state events
    pub send_state_event: Vec<EventFilter>,
    pub read_state_event: Vec<EventFilter>,

    pub always_on_screen: bool, // "m.always_on_screen",

    pub requires_client: bool,
}

impl Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut capability_list: Vec<String> = vec![];
        if self.screenshot {
            capability_list.push("m.capability.screenshot".to_owned());
        }
        if self.always_on_screen {
            capability_list.push("m.always_on_screen".to_owned());
        }
        if self.always_on_screen {
            capability_list.push("m.always_on_screen".to_owned());
        }
        if self.requires_client {
            capability_list.push("io.element.requires_client".to_owned());
        }

        for event_type in &self.send_room_event {
            let filter = serde_json::to_string(event_type).unwrap();
            capability_list.push(format!("org.matrix.msc2762.m.send.event{}", filter));
        }

        for event_type in &self.read_room_event {
            let filter = serde_json::to_string(event_type).unwrap();
            capability_list.push(format!("org.matrix.msc2762.m.receive.event{}", filter));
        }

        for event_type in &self.send_state_event {
            let filter = serde_json::to_string(event_type).unwrap();
            capability_list.push(format!("org.matrix.msc2762.m.send.state_event{}", filter));
        }

        for event_type in &self.read_state_event {
            let filter = serde_json::to_string(event_type).unwrap();
            capability_list.push(format!("org.matrix.msc2762.m.receive.state_event{}", filter));
        }

        capability_list.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Options {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let capability_list = Vec::<String>::deserialize(deserializer)?;
        let mut capbilities = Options::default();

        for capability in capability_list {
            if capability == "m.capability.screenshot" {
                capbilities.screenshot = true;
            }
            if capability == "m.always_on_screen" {
                capbilities.always_on_screen = true;
            }
            if capability == "io.element.requires_client" {
                capbilities.requires_client = true;
            }
            if capability.starts_with("org.matrix.msc2762.m.send.event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    capbilities.send_room_event.push(serde_json::from_str(cap_split[1]).unwrap());
                }
            }
            if capability.starts_with("org.matrix.msc2762.m.receive.event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    capbilities.read_room_event.push(serde_json::from_str(cap_split[1]).unwrap());
                }
            }
            if capability.starts_with("org.matrix.msc2762.m.send.state_event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    capbilities.send_state_event.push(serde_json::from_str(cap_split[1]).unwrap());
                }
            }
            if capability.starts_with("org.matrix.msc2762.m.receive.state_event") {
                let cap_split: Vec<&str> = capability.split(":").collect();
                if cap_split.len() > 1 {
                    capbilities.read_state_event.push(serde_json::from_str(cap_split[1]).unwrap());
                }
            }
        }

        Ok(capbilities)
    }
}

// Event Filters

pub trait Filter {
    fn allow_event(
        &self,
        message_type: &String,
        state_key: &Option<String>,
        content: &serde_json::Value,
    ) -> bool;
}
#[derive(Debug, Default, Clone)]
pub struct EventFilter {
    event_type: String,
    msgtype: Option<String>,
}
impl Filter for EventFilter {
    fn allow_event(
        &self,
        message_type: &String,
        state_key: &Option<String>,
        content: &serde_json::Value,
    ) -> bool {
        if message_type == &self.event_type {
            if let Some(msgtype) = self.msgtype.clone() {
                if message_type == "m.room.message" {
                    if content
                        .get("msgtype")
                        .unwrap_or(&serde_json::to_value("").unwrap())
                        .to_string()
                        == msgtype
                    {
                        return true;
                    }
                }
                return false;
            }
            return true;
        }
        return false;
    }
}
#[derive(Debug, Default, Clone)]
pub struct EventFilterAllowAll {}
impl Filter for EventFilterAllowAll {
    fn allow_event(
        &self,
        message_type: &String,
        state_key: &Option<String>,
        content: &serde_json::Value,
    ) -> bool {
        true
    }
}
#[derive(Debug, Default, Clone)]
pub struct StateEventFilter {
    event_type: String,
    state_key: Option<String>,
}
impl Filter for StateEventFilter {
    fn allow_event(
        &self,
        message_type: &String,
        state_key: &Option<String>,
        content: &serde_json::Value,
    ) -> bool {
        if message_type == &self.event_type {
            if let (Some(filter_key), Some(ev_key)) = (self.state_key.clone(), state_key.clone()) {
                return filter_key == ev_key;
            }
            return true;
        }
        return false;
    }
}
impl Serialize for EventFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut string = format!("{}", self.event_type);
        if let Some(msgtype) = &self.msgtype {
            string = format!("{}#{}", string, msgtype);
        }
        serializer.serialize_str(&string)
    }
}
impl Serialize for StateEventFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut string = format!(":{}", self.event_type);
        if let Some(state_key) = &self.state_key {
            string = format!("{}#{}", string, state_key);
        }
        serializer.serialize_str(&string)
    }
}

impl<'de> Deserialize<'de> for StateEventFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let des_string = String::deserialize(deserializer)?;
        let split: Vec<&str> = des_string.split("#").collect();
        let ev_type = split[0].to_owned();
        let mut state_key: Option<String> = None;
        if split.len() > 1 {
            state_key = Some(split[1].to_owned())
        }
        Ok(StateEventFilter { event_type: ev_type, state_key: state_key })
    }
}
impl<'de> Deserialize<'de> for EventFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let des_string = String::deserialize(deserializer)?;
        let split: Vec<&str> = des_string.split("#").collect();
        let ev_type = split[0].to_owned();
        let mut msgtype: Option<String> = None;
        if split.len() > 1 {
            msgtype = Some(split[1].to_owned())
        }
        Ok(EventFilter { event_type: ev_type, msgtype: msgtype })
    }
}
