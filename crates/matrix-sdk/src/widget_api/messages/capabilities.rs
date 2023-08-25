use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::from_str;
use std::fmt::Debug;

use super::{from_widget::SendEventRequest, MatrixEvent};

const SEND_EVENT: &str = "org.matrix.msc2762.m.send.event";
const READ_EVENT: &str = "org.matrix.msc2762.m.receive.event";
const SEND_STATE: &str = "org.matrix.msc2762.m.send.state_event";
const READ_STATE: &str = "org.matrix.msc2762.m.receive.state_event";
const SCREENSHOT: &str = "m.capability.screenshot";
const ALWAYS_ON_SCREEN: &str = "m.always_on_screen";
const REQUIRES_CLIENT: &str = "io.element.requires_client";

#[derive(Debug, Default, Clone)]
pub struct Options {
    pub send_filter: Vec<Filter>,
    pub read_filter: Vec<Filter>,
    pub screenshot: bool,
    pub always_on_screen: bool,
    pub requires_client: bool,
}

impl Serialize for Options {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut capability_list: Vec<String> = vec![];

        let caps = vec![self.screenshot, self.always_on_screen, self.requires_client];
        let strs = vec![SCREENSHOT, ALWAYS_ON_SCREEN, REQUIRES_CLIENT];

        caps.iter()
            .zip(strs.iter())
            .filter(|(c, s)| **c)
            .for_each(|(c, s)| capability_list.push((*s).to_owned()));

        let s = self.send_filter.clone();
        let send =
            s.into_iter().map(|x| (if x.is_state_filter() { SEND_STATE } else { SEND_EVENT }, x));
        let r = self.read_filter.clone();
        let read =
            r.into_iter().map(|x| (if x.is_state_filter() { READ_STATE } else { READ_EVENT }, x));

        for (base, filter) in send.chain(read) {
            let ext =
                filter.capability_extension().map_err(|e| ser::Error::custom(e.to_string()))?;
            capability_list.push(base.to_owned() + &ext);
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
        let mut capabilities = Options::default();
        let err_m = |e: serde_json::Error| de::Error::custom(e.to_string());
        for capability in capability_list {
            match &capability.split(":").collect::<Vec<_>>().as_slice() {
                [SCREENSHOT] => capabilities.screenshot = true,
                [ALWAYS_ON_SCREEN] => capabilities.always_on_screen = true,
                [REQUIRES_CLIENT] => capabilities.requires_client = true,

                [SEND_EVENT] => capabilities.send_filter.push(Filter::AllowAllMessage),
                [SEND_EVENT, rest] => {
                    capabilities.send_filter.push(Filter::Timeline(from_str(rest).map_err(err_m)?))
                }
                [READ_EVENT] => capabilities.read_filter.push(Filter::AllowAllMessage),
                [READ_EVENT, rest] => {
                    capabilities.read_filter.push(Filter::Timeline(from_str(rest).map_err(err_m)?));
                }

                [SEND_STATE] => capabilities.send_filter.push(Filter::AllowAllState),
                [SEND_STATE, rest] => {
                    capabilities.send_filter.push(Filter::State(from_str(rest).map_err(err_m)?))
                }
                [READ_STATE] => capabilities.read_filter.push(Filter::AllowAllState),
                [READ_STATE, rest] => {
                    capabilities.read_filter.push(Filter::State(from_str(rest).map_err(err_m)?));
                }
                _ => {}
            }
        }
        Ok(capabilities)
    }
}

// Event Filters
#[derive(Debug, Clone)]
pub enum Filter {
    Timeline(MessageFilter),
    State(StateFilter),
    AllowAllMessage,
    AllowAllState,
}

impl EventFilter for Filter {
    fn allow(&self, input: FilterInput<'_>) -> bool {
        match self {
            Filter::Timeline(f) => f.allow(input),
            Filter::State(f) => f.allow(input),
            Filter::AllowAllMessage => input.state_key.is_none(),
            Filter::AllowAllState => input.state_key.is_some(),
        }
    }
}

impl Filter {
    pub fn is_state_filter(&self) -> bool {
        match self {
            Filter::Timeline(_) | Filter::AllowAllMessage => false,
            Filter::State(_) | Filter::AllowAllState => true,
        }
    }

    fn capability_extension(&self) -> Result<String, serde_json::Error> {
        match self {
            Filter::State(s_filter) => serde_json::to_string(s_filter),
            Filter::Timeline(t_filter) => serde_json::to_string(t_filter),
            Filter::AllowAllMessage | Filter::AllowAllState => Ok("".to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterInput<'a> {
    pub event_type: &'a str,
    pub state_key: Option<&'a str>,
    pub msgtype: Option<&'a str>,
}
impl<'a> FilterInput<'a> {
    fn new(
        ev_type: &'a String,
        state_key: &'a Option<String>,
        content: &'a serde_json::Value,
    ) -> Self {
        Self {
            event_type: ev_type.as_str(),
            state_key: state_key.as_ref().map(|s| s.as_str()),
            msgtype: content.get("msgtype").and_then(|v| v.as_str()),
        }
    }
}

impl<'a> From<&'a MatrixEvent> for FilterInput<'a> {
    fn from(e: &'a MatrixEvent) -> Self {
        Self::new(&e.event_type, &e.state_key, &e.content)
    }
}

impl<'a> From<&'a SendEventRequest> for FilterInput<'a> {
    fn from(r: &'a SendEventRequest) -> Self {
        Self::new(&r.message_type, &r.state_key, &r.content)
    }
}

pub trait EventFilter {
    fn allow(&self, input: FilterInput<'_>) -> bool;
}

#[derive(Debug, Default, Clone)]
pub struct MessageFilter {
    event_type: String,
    msgtype: Option<String>,
}

impl EventFilter for MessageFilter {
    fn allow(&self, input: FilterInput<'_>) -> bool {
        if self.event_type != input.event_type {
            return false;
        }

        let Some(allowed_type) = self.msgtype.as_ref() else {
            return true;
        };

        if input.event_type != "m.room.message" {
            return false;
        }

        input.msgtype.map(|t| t == allowed_type).unwrap_or(false)
    }
}

#[derive(Debug, Default, Clone)]
pub struct StateFilter {
    event_type: String,
    state_key: Option<String>,
}

impl EventFilter for StateFilter {
    fn allow(&self, input: FilterInput<'_>) -> bool {
        if &self.event_type != input.event_type {
            return false;
        }

        self.state_key
            .as_ref()
            .zip(input.state_key)
            .map(|(expected, passed)| expected == passed)
            .unwrap_or(false)
    }
}

fn from_type_and_suffix<S: Serializer>(
    s: S,
    ev_type: &str,
    suffix: Option<&str>,
) -> Result<S::Ok, S::Error> {
    let serialized_string = format!("{}#{}", ev_type, suffix.unwrap_or(""));
    s.serialize_str(&serialized_string)
}
impl Serialize for MessageFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        from_type_and_suffix(serializer, &self.event_type, self.msgtype.as_deref())
    }
}

impl Serialize for StateFilter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        from_type_and_suffix(serializer, &self.event_type, self.state_key.as_deref())
    }
}

fn to_type_and_suffix<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<(String, Option<String>), D::Error> {
    let event_type = String::deserialize(deserializer)?;
    
    if let Some((ev_type, msg_type)) = event_type.clone().split_once("#") {
        Ok((ev_type.to_owned(), Some(msg_type.to_owned())))
    } else {
        Ok((event_type, None))
    }
}

impl<'de> Deserialize<'de> for StateFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (event_type, state_key) = to_type_and_suffix(deserializer)?;
        Ok(StateFilter { event_type, state_key })
    }
}

impl<'de> Deserialize<'de> for MessageFilter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (event_type, msgtype) = to_type_and_suffix(deserializer)?;
        Ok(MessageFilter { event_type, msgtype })
    }
}
