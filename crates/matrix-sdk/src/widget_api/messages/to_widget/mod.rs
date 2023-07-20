use super::super::capabilities::Options as CapabilityRequest;

pub mod actions;

pub use self::actions::ToWidgetAction;
pub trait ToWidget {
    type Response;
}

pub struct SendMeCapabilities;
impl ToWidget for SendMeCapabilities {
    type Response = CapabilityRequest;
}

pub type CapabilitiesUpdated = CapabilityRequest;
impl ToWidget for CapabilitiesUpdated {
    type Response = ();
}
