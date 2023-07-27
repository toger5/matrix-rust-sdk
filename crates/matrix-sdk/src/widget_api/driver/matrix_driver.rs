use std::collections::HashSet;
// use uuid::Uuid;

use futures_util::future::Join;
use ruma::events::room::message::SyncRoomMessageEvent;

use crate::{
    room::{Joined, Room},
    Client,
};


pub enum ReadDirection {
    Forward,
    Backwards,
}
pub trait MatrixDriver {

    /**
     * Sends an event into a room. If `roomId` is falsy, the client should send the event
     * into the room the user is currently looking at. The widget API will have already
     * verified that the widget is capable of sending the event to that room.
     * @param {string} eventType The event type to be sent.
     * @param {*} content The content for the event.
     * @param {string|null} stateKey The state key if this is a state event, otherwise null.
     * May be an empty string.
     * @param {string|null} roomId The room ID to send the event to. If falsy, the room the
     * user is currently looking at.
     * @returns {Promise<ISendEventDetails>} Resolves when the event has been sent with
     * details of that event.
     * @throws Rejected when the event could not be sent.
     */
    fn send_event(eventType: &str, content: serde_json::Value, stateKey: &str, roomId: &str);

    /**
     * Sends a to-device event. The widget API will have already verified that the widget
     * is capable of sending the event.
     * @param {string} eventType The event type to be sent.
     * @param {boolean} encrypted Whether to encrypt the message contents.
     * @param {Object} contentMap A map from user ID and device ID to event content.
     * @returns {Promise<void>} Resolves when the event has been sent.
     * @throws Rejected when the event could not be sent.
     */
    fn send_to_device(
        eventType: &str,
        encrypted: bool,
        contentMap: serde_json::Value, /*{ [userId: string]: { [deviceId: string]: object } }*/
    );

    /**
     * Reads all events of the given type, and optionally `msgtype` (if applicable/defined),
     * the user has access to. The widget API will have already verified that the widget is
     * capable of receiving the events. Less events than the limit are allowed to be returned,
     * but not more. If `roomIds` is supplied, it may contain `Symbols.AnyRoom` to denote that
     * `limit` in each of the client's known rooms should be returned. When `null`, only the
     * room the user is currently looking at should be considered.
     * @param eventType The event type to be read.
     * @param msgtype The msgtype of the events to be read, if applicable/defined.
     * @param limit The maximum number of events to retrieve per room. Will be zero to denote "as many
     * as possible".
     * @param roomIds When null, the user's currently viewed room. Otherwise, the list of room IDs
     * to look within, possibly containing Symbols.AnyRoom to denote all known rooms.
     * @returns {Promise<IRoomEvent[]>} Resolves to the room events, or an empty array.
     */
    fn read_room_events(eventType: &str, msgtype: &str, limit: u32, roomIds: Vec<String>);

    /**
     * Reads all events of the given type, and optionally state key (if applicable/defined),
     * the user has access to. The widget API will have already verified that the widget is
     * capable of receiving the events. Less events than the limit are allowed to be returned,
     * but not more. If `roomIds` is supplied, it may contain `Symbols.AnyRoom` to denote that
     * `limit` in each of the client's known rooms should be returned. When `null`, only the
     * room the user is currently looking at should be considered.
     * @param eventType The event type to be read.
     * @param stateKey The state key of the events to be read, if applicable/defined.
     * @param limit The maximum number of events to retrieve. Will be zero to denote "as many
     * as possible".
     * @param roomIds When null, the user's currently viewed room. Otherwise, the list of room IDs
     * to look within, possibly containing Symbols.AnyRoom to denote all known rooms.
     * @returns {Promise<IRoomEvent[]>} Resolves to the state events, or an empty array.
     */
    fn read_state_events(eventType: &str, stateKey: &str, limit: u32, roomIds: Vec<String>);

    /**
     * Reads all events that are related to a given event. The widget API will
     * have already verified that the widget is capable of receiving the event,
     * or will make sure to reject access to events which are returned from this
     * function, but are not capable of receiving. If `relationType` or `eventType`
     * are set, the returned events should already be filtered. Less events than
     * the limit are allowed to be returned, but not more.
     * @param eventId The id of the parent event to be read.
     * @param roomId The room to look within. When undefined, the user's
     * currently viewed room.
     * @param relationType The relationship type of child events to search for.
     * When undefined, all relations are returned.
     * @param eventType The event type of child events to search for. When undefined,
     * all related events are returned.
     * @param from The pagination token to start returning results from, as
     * received from a previous call. If not supplied, results start at the most
     * recent topological event known to the server.
     * @param to The pagination token to stop returning results at. If not
     * supplied, results continue up to limit or until there are no more events.
     * @param limit The maximum number of events to retrieve per room. If not
     * supplied, the server will apply a default limit.
     * @param direction The direction to search for according to MSC3715
     * @returns Resolves to the room relations.
     */
    fn read_event_relations(
        eventId: &str,
        roomId: &str,
        relationType: &str,
        eventType: &str,
        from: &str,
        to: &str,
        limit: u32,
        direction: ReadDirection,
    );

    /// Asks the user for permission to validate their identity through OpenID Connect. The
    /// interface for this function is an observable which accepts the state machine of the
    /// OIDC exchange flow. For example, if the client/user blocks the request then it would
    /// feed back a `{state: Blocked}` into the observable. Similarly, if the user already
    /// approved the widget then a `{state: Allowed}` would be fed into the observable alongside
    /// the token itself. If the client is asking for permission, it should feed in a
    /// `{state: PendingUserConfirmation}` followed by the relevant Allowed or Blocked state.
    ///
    /// The widget API will reject the widget's request with an error if this contract is not
    /// met properly. By default, the widget driver will block all OIDC requests.
    /// @param {SimpleObservable<IOpenIDUpdate>} observer The observable to feed updates into.
    fn ask_open_id(/*observer: SimpleObservable<IOpenIDUpdate>*/);

    /// Polls for TURN server data, yielding an initial set of credentials as soon as possible, and
    /// thereafter yielding new credentials whenever the previous ones expire. The widget API will
    /// have already verified that the widget has permission to access TURN servers.
    /// @yields {ITurnServer} The TURN server URIs and credentials currently available to the client.
    fn get_turn_servers();

}

#[derive(Clone, Debug)]
pub struct RustSdkMatrixDriver {
    pub room: Joined,
}
impl RustSdkMatrixDriver {
    pub fn new(room: Joined, widget_id: String) -> Self {
        let matrix_driver = RustSdkMatrixDriver { room };
        let driver_room = matrix_driver.room.clone();
        let room_message_callback = |ev: SyncRoomMessageEvent, room: Room, client: Client| async move {
            todo!();
        };
        driver_room.inner.client.add_event_handler(room_message_callback);
        matrix_driver
    }
}
impl MatrixDriver for RustSdkMatrixDriver {
    fn ask_open_id(/*observer: SimpleObservable<IOpenIDUpdate>*/) {
        unimplemented!()
    }
    fn get_turn_servers() {
        unimplemented!()
    }
    fn read_event_relations(
        eventId: &str,
        roomId: &str,
        relationType: &str,
        eventType: &str,
        from: &str,
        to: &str,
        limit: u32,
        direction: ReadDirection,
    ) {
        unimplemented!()
    }
    fn read_room_events(eventType: &str, msgtype: &str, limit: u32, roomIds: Vec<String>) {
        unimplemented!()
    }
    fn read_state_events(eventType: &str, stateKey: &str, limit: u32, roomIds: Vec<String>) {
        unimplemented!()
    }
    fn send_event(eventType: &str, content: serde_json::Value, stateKey: &str, roomId: &str) {
        unimplemented!()
    }
    fn send_to_device(
        eventType: &str,
        encrypted: bool,
        contentMap: serde_json::Value, /*{ [userId: string]: { [deviceId: string]: object } }*/
    ) {
        unimplemented!()
    }
}
