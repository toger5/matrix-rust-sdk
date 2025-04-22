// Copyright 2023 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ruma::{
    events::{
        AnyMessageLikeEventContent, AnyTimelineEvent, MessageLikeEventType, StateEventType,
        TimelineEventType,
    },
    serde::Raw,
};
use serde::Deserialize;
use tracing::debug;

use super::machine::SendEventRequest;

/// A Filter for Matrix events. That is used to decide
/// if a given event can be sent to the widget and if a widgets
/// is allowed to send an event to to a matrix room or not.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Filter {
    /// Filter for message-like events.
    MessageLike(MessageLikeEventFilter),
    /// Filter for state events.
    State(StateEventFilter),
}

impl Filter {
    /// Checks if this filter matches with the given filter_input.
    /// A filter input can be create by using the `From` trait on FilterInput
    /// for [`Raw<AnyTimelineEvent>`] or [`SendEventRequest`].
    pub(super) fn matches(&self, filter_input: &FilterInput) -> bool {
        match self {
            Self::MessageLike(filter) => filter.matches(filter_input),
            Self::State(filter) => filter.matches(filter_input),
        }
    }
    /// Returns the event type that this filter is configured to match.
    ///
    /// This method provides a string representation of the event type
    /// associated with the filter.
    pub(super) fn filter_event_type(&self) -> String {
        match self {
            Self::MessageLike(filter) => filter.filter_event_type(),
            Self::State(filter) => filter.filter_event_type(),
        }
    }
}

/// Filter for message-like events.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MessageLikeEventFilter {
    /// Matches message-like events with the given `type`.
    WithType(MessageLikeEventType),
    /// Matches `m.room.message` events with the given `msgtype`.
    RoomMessageWithMsgtype(String),
}

impl MessageLikeEventFilter {
    fn matches(&self, filter_input: &FilterInput) -> bool {
        let FilterInput::MessageLike(message_like_filter_input) = filter_input else {
            return false;
        };
        match self {
            Self::WithType(filter_event_type) => {
                &message_like_filter_input.event_type == filter_event_type
            }
            Self::RoomMessageWithMsgtype(msgtype) => {
                message_like_filter_input.event_type == MessageLikeEventType::RoomMessage
                    && message_like_filter_input.content.msgtype.as_ref() == Some(msgtype)
            }
        }
    }

    fn filter_event_type(&self) -> String {
        match self {
            Self::WithType(filter_event_type) => filter_event_type.to_string(),
            Self::RoomMessageWithMsgtype(_) => MessageLikeEventType::RoomMessage.to_string(),
        }
    }
}

/// Filter for state events.
#[derive(Clone, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum StateEventFilter {
    /// Matches state events with the given `type`, regardless of `state_key`.
    WithType(StateEventType),
    /// Matches state events with the given `type` and `state_key`.
    WithTypeAndStateKey(StateEventType, String),
}

impl StateEventFilter {
    fn matches(&self, filter_input: &FilterInput) -> bool {
        let FilterInput::State(state_filter_input) = filter_input else {
            return false;
        };

        match self {
            StateEventFilter::WithType(filter_type) => {
                &state_filter_input.event_type == filter_type
            }
            StateEventFilter::WithTypeAndStateKey(event_type, filter_state_key) => {
                &state_filter_input.event_type == event_type
                    && state_filter_input.state_key == *filter_state_key
            }
        }
    }
    fn filter_event_type(&self) -> String {
        match self {
            Self::WithType(filter_event_type) => filter_event_type.to_string(),
            Self::WithTypeAndStateKey(event_type, _) => event_type.to_string(),
        }
    }
}

// Filter input:

/// The input data for the filter. This can either be constructed from a
/// [`Raw<AnyTimelineEvent>`] or a [`SendEventRequest`].
#[derive(Debug)]
pub enum FilterInput {
    State(FilterInputState),
    MessageLike(FilterInputMessageLike),
}

impl FilterInput {
    pub fn message_like(event_type: String) -> Self {
        Self::MessageLike(FilterInputMessageLike {
            event_type: event_type.into(),
            content: MessageLikeFilterEventContent { msgtype: None },
        })
    }

    pub(super) fn message_with_msgtype(msgtype: String) -> Self {
        Self::MessageLike(FilterInputMessageLike {
            event_type: MessageLikeEventType::RoomMessage,
            content: MessageLikeFilterEventContent { msgtype: Some(msgtype) },
        })
    }

    pub fn state(event_type: String, state_key: String) -> Self {
        Self::State(FilterInputState { event_type: event_type.into(), state_key })
    }
}

/// Filter input data that is used for a [`FilterInput::State`] filter.
#[derive(Debug, Deserialize)]
pub struct FilterInputState {
    #[serde(rename = "type")]
    pub(super) event_type: StateEventType,
    pub(super) state_key: String,
}

// Filter input message like:
#[derive(Debug, Default, Deserialize)]
pub(super) struct MessageLikeFilterEventContent {
    pub(super) msgtype: Option<String>,
}

#[derive(Debug)]
pub struct FilterInputMessageLike {
    pub(super) event_type: MessageLikeEventType,
    pub(super) content: MessageLikeFilterEventContent,
}

/// Create a filter input based on [`AnyTimelineEvent`].
/// This will create a [`FilterInput::State`] or [`FilterInput::MessageLike`]
/// depending on the event type.
impl TryFrom<Raw<AnyTimelineEvent>> for FilterInput {
    type Error = serde_json::Error;
    fn try_from(raw_event: Raw<AnyTimelineEvent>) -> Result<Self, Self::Error> {
        // make sure `raw_event` actually was a timeline event
        let timeline_event = raw_event.deserialize()?;
        match timeline_event {
            AnyTimelineEvent::MessageLike(message_like) => {
                let msgtype = if let Some(AnyMessageLikeEventContent::RoomMessage(r)) =
                    message_like.original_content()
                {
                    Some(r.msgtype().to_owned())
                } else {
                    None
                };
                let input_content = FilterInputMessageLike {
                    event_type: message_like.event_type(),
                    content: MessageLikeFilterEventContent { msgtype },
                };
                Ok(FilterInput::MessageLike(input_content))
            }
            AnyTimelineEvent::State(s) => Ok(FilterInput::State(FilterInputState {
                event_type: s.event_type(),
                state_key: s.state_key().to_owned(),
            })),
        }
    }
}

impl From<&SendEventRequest> for FilterInput {
    fn from(request: &SendEventRequest) -> Self {
        match &request.state_key {
            None => match request.event_type {
                TimelineEventType::RoomMessage => {
                    if let Some(msgtype) =
                        serde_json::from_str::<MessageLikeFilterEventContent>(request.content.get())
                            .unwrap_or_else(|e| {
                                debug!("Failed to deserialize event content for filter: {e}");
                                // Fallback to empty content is safe because there is no filter
                                // that matches with it when it otherwise wouldn't.
                                Default::default()
                            })
                            .msgtype
                    {
                        FilterInput::message_with_msgtype(msgtype)
                    } else {
                        FilterInput::message_like(TimelineEventType::RoomMessage.to_string())
                    }
                }
                _ => FilterInput::message_like(request.event_type.to_string()),
            },
            Some(state_key) => {
                FilterInput::state(request.event_type.to_string(), state_key.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use ruma::events::{MessageLikeEventType, StateEventType, TimelineEventType};

    use super::{
        Filter, FilterInput, FilterInputMessageLike, MessageLikeEventFilter, StateEventFilter,
    };
    use crate::widget::filter::MessageLikeFilterEventContent;

    fn message_event(event_type: MessageLikeEventType) -> FilterInput {
        FilterInput::MessageLike(FilterInputMessageLike { event_type, content: Default::default() })
    }

    // Tests against an `m.room.message` filter with `msgtype = m.text`
    fn room_message_text_event_filter() -> Filter {
        Filter::MessageLike(MessageLikeEventFilter::RoomMessageWithMsgtype("m.text".to_owned()))
    }

    #[test]
    fn test_text_event_filter_matches_text_event() {
        assert!(room_message_text_event_filter()
            .matches(&FilterInput::message_with_msgtype("m.text".to_owned())));
    }

    #[test]
    fn test_text_event_filter_does_not_match_image_event() {
        assert!(!room_message_text_event_filter()
            .matches(&FilterInput::message_with_msgtype("m.image".to_owned())));
    }

    #[test]
    fn test_text_event_filter_does_not_match_custom_event_with_msgtype() {
        assert!(!room_message_text_event_filter().matches(&FilterInput::MessageLike(
            FilterInputMessageLike {
                event_type: "io.element.message".into(),
                content: MessageLikeFilterEventContent { msgtype: Some("m.text".to_owned()) }
            }
        )));
    }

    // Tests against an `m.reaction` filter
    fn reaction_event_filter() -> Filter {
        Filter::MessageLike(MessageLikeEventFilter::WithType(MessageLikeEventType::Reaction))
    }

    #[test]
    fn test_reaction_event_filter_matches_reaction() {
        assert!(reaction_event_filter().matches(&message_event(MessageLikeEventType::Reaction)));
    }

    #[test]
    fn test_reaction_event_filter_does_not_match_room_message() {
        assert!(!reaction_event_filter()
            .matches(&FilterInput::message_with_msgtype("m.text".to_owned())));
    }

    #[test]
    fn test_reaction_event_filter_does_not_match_state_event_any_key() {
        assert!(!reaction_event_filter()
            .matches(&FilterInput::state("m.reaction".to_owned(), "".to_owned())));
    }

    // Tests against an `m.room.member` filter with `state_key = "@self:example.me"`
    fn self_member_event_filter() -> Filter {
        Filter::State(StateEventFilter::WithTypeAndStateKey(
            StateEventType::RoomMember,
            "@self:example.me".to_owned(),
        ))
    }

    #[test]
    fn test_self_member_event_filter_matches_self_member_event() {
        assert!(self_member_event_filter().matches(&FilterInput::state(
            TimelineEventType::RoomMember.to_string(),
            "@self:example.me".to_owned()
        )));
    }

    #[test]
    fn test_self_member_event_filter_does_not_match_somebody_elses_member_event() {
        assert!(!self_member_event_filter().matches(&FilterInput::state(
            TimelineEventType::RoomMember.to_string(),
            "@somebody_else.example.me".to_owned()
        )));
    }

    #[test]
    fn self_member_event_filter_does_not_match_unrelated_state_event_with_same_state_key() {
        assert!(!self_member_event_filter().matches(&FilterInput::state(
            TimelineEventType::from("io.element.test_state_event").to_string(),
            "@self.example.me".to_owned()
        )));
    }

    #[test]
    fn test_self_member_event_filter_does_not_match_reaction_event() {
        assert!(!self_member_event_filter().matches(&message_event(MessageLikeEventType::Reaction)));
    }

    #[test]
    fn test_self_member_event_filter_only_matches_specific_state_key() {
        assert!(!self_member_event_filter()
            .matches(&FilterInput::state(StateEventType::RoomMember.to_string(), "".to_owned())));
    }

    // Tests against an `m.room.member` filter with any `state_key`.
    fn member_event_filter() -> Filter {
        Filter::State(StateEventFilter::WithType(StateEventType::RoomMember))
    }

    #[test]
    fn test_member_event_filter_matches_some_member_event() {
        assert!(member_event_filter().matches(&FilterInput::state(
            TimelineEventType::RoomMember.to_string(),
            "@foo.bar.baz".to_owned()
        )));
    }

    #[test]
    fn test_member_event_filter_does_not_match_room_name_event() {
        assert!(!member_event_filter()
            .matches(&FilterInput::state(TimelineEventType::RoomName.to_string(), "".to_owned())));
    }

    #[test]
    fn test_member_event_filter_does_not_match_reaction_event() {
        assert!(!member_event_filter().matches(&message_event(MessageLikeEventType::Reaction)));
    }

    #[test]
    fn test_member_event_filter_matches_any_state_key() {
        assert!(member_event_filter()
            .matches(&FilterInput::state(StateEventType::RoomMember.to_string(), "".to_owned())));
    }

    // Tests against an `m.room.topic` filter with `state_key = ""`
    fn topic_event_filter() -> Filter {
        Filter::State(StateEventFilter::WithTypeAndStateKey(
            StateEventType::RoomTopic,
            "".to_owned(),
        ))
    }

    #[test]
    fn test_topic_event_filter_does_match() {
        assert!(topic_event_filter()
            .matches(&FilterInput::state(StateEventType::RoomTopic.to_string(), "".to_owned())));
    }

    // Tests against an `m.room.message` filter with `msgtype = m.custom`
    fn room_message_custom_event_filter() -> Filter {
        Filter::MessageLike(MessageLikeEventFilter::RoomMessageWithMsgtype("m.custom".to_owned()))
    }

    // Tests against an `m.room.message` filter without a `msgtype`
    fn room_message_filter() -> Filter {
        Filter::MessageLike(MessageLikeEventFilter::WithType(MessageLikeEventType::RoomMessage))
    }

    #[test]
    fn test_reaction_event_type_does_not_match_room_message_text_event_filter() {
        assert!(!room_message_text_event_filter()
            .matches(&FilterInput::message_like(MessageLikeEventType::Reaction.to_string())));
    }

    #[test]
    fn test_room_message_event_without_msgtype_does_not_match_custom_msgtype_filter() {
        assert!(!room_message_custom_event_filter()
            .matches(&FilterInput::message_like(MessageLikeEventType::RoomMessage.to_string())));
    }

    #[test]
    fn test_reaction_event_type_does_not_match_room_message_custom_event_filter() {
        assert!(!room_message_custom_event_filter()
            .matches(&FilterInput::message_like(MessageLikeEventType::Reaction.to_string())));
    }

    #[test]
    fn test_room_message_event_type_matches_room_message_event_filter() {
        assert!(room_message_filter()
            .matches(&FilterInput::message_like(MessageLikeEventType::RoomMessage.to_string())));
    }

    #[test]
    fn test_reaction_event_type_does_not_match_room_message_event_filter() {
        assert!(!room_message_filter()
            .matches(&FilterInput::message_like(MessageLikeEventType::Reaction.to_string())));
    }
}
