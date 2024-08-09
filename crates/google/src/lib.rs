mod error;
mod oauth;
mod codegen;

pub use error::{Error, Result};
pub use oauth::ServiceAccountOAuthManager;

pub use codegen::discovery::discover_apis;
