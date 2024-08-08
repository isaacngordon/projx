mod error;
mod oauth;
pub use error::{Error, Result};

pub const DISCOVERY_API_ENDPOINT: &str = "https://discovery.googleapis.com/discovery/v1/apis";
