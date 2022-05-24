//! Types related to events.

use crate::prelude::*;

/// Who can see a room's history.
#[cfg_attr(feature = "js", wasm_bindgen)]
#[cfg_attr(feature = "nodejs", napi)]
#[derive(Debug)]
pub enum HistoryVisibility {
    /// Previous events are accessible to newly joined members from
    /// the point they were invited onwards.
    ///
    /// Events stop being accessible when the member's state changes
    /// to something other than *invite* or *join*.
    Invited,

    /// Previous events are accessible to newly joined members from
    /// the point they joined the room onwards.
    ///
    /// Events stop being accessible when the member's state changes
    /// to something other than *join*.
    Joined,

    /// Previous events are always accessible to newly joined members.
    ///
    /// All events in the room are accessible, even those sent when
    /// the member was not a part of the room.
    Shared,

    /// All events while this is the `HistoryVisibility` value may be
    /// shared by any participating homeserver with anyone, regardless
    /// of whether they have ever joined the room.
    WorldReadable,
}

impl From<HistoryVisibility> for ruma::events::room::history_visibility::HistoryVisibility {
    fn from(value: HistoryVisibility) -> Self {
        use HistoryVisibility::*;

        match value {
            Invited => Self::Invited,
            Joined => Self::Joined,
            Shared => Self::Shared,
            WorldReadable => Self::WorldReadable,
        }
    }
}

impl Into<HistoryVisibility> for ruma::events::room::history_visibility::HistoryVisibility {
    fn into(self) -> HistoryVisibility {
        use HistoryVisibility::*;

        match self {
            Self::Invited => Invited,
            Self::Joined => Joined,
            Self::Shared => Shared,
            Self::WorldReadable => WorldReadable,
            _ => unreachable!("Unknown variant"),
        }
    }
}
