use serde::{Serialize, Deserialize};

pub mod capabilities;
pub mod open_id_credentials;
pub mod supported_api_versions;

#[derive(Serialize, Deserialize, Debug)]
pub struct WidgetMessageEmptyData {}

