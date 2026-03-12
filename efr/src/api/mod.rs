mod datetime;
mod envelope;
mod error;
mod json;
mod multipart;
mod secure_efr_request;
mod security;
mod traits;
mod tyler_header;
mod uuid;

pub(crate) use envelope::Envelope;
pub use error::EfrError;
pub use json::json;
pub use multipart::MultiPartRequest;
pub(crate) use multipart::{MultiPartRequestBuilder, MultiPartResponse};
pub(crate) use secure_efr_request::{SecureEfrRequest, insecure_request};
pub(crate) use security::SecurityHeader;
pub(crate) use traits::Xml;
pub use traits::{EfrRequest, EfrResponse};
pub(crate) use tyler_header::TylerHeader;
pub(crate) use uuid::HEX;

#[cfg(feature = "metadata")]
mod metadata;

#[cfg(feature = "metadata")]
pub use metadata::{Environment, EnvironmentError, Metadata, State, StateError};

pub(crate) mod serde_datetime;
