use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CapabilitiesResponse{
    capabilities: Vec<String>
}

