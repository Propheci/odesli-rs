use reqwest::StatusCode;

/// Various error kinds that the library can return.
#[derive(Debug)]
pub enum OdesliError {
    /// An unknown [`crate::EntityType`] value was passed.
    ///
    /// Accepted values are "song" and "album".
    UnknownEntityType(String),
    /// An unknown [`crate::Platform`] value was passed.
    UnknownPlatform(String),
    /// An unknown [`crate::APIProvider`] value was passed.
    UnknownAPIProvider(String),
    /// Failed to JSON parse the response from Odesli API.
    ParseError {
        /// The error encountered while parsing the body.
        error: String,
        /// The response body (will be printed only in Debug print).
        body: String,
    },
    /// Received non-200 status code from Odesli API.
    Non200StatusCode {
        /// The status code returned by Odesli.
        status_code: StatusCode,
        /// The response body (will be printed only in Debug print).
        body: String,
    },
    /// Error returned by [`reqwest::Client`] on sending a request.
    ReqwestError(reqwest::Error),
}

impl std::fmt::Display for OdesliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownEntityType(entity) => format!("Unknown EntityType: {entity}",),
                Self::UnknownPlatform(platform) => format!("Unknown Platform: {platform}"),
                Self::UnknownAPIProvider(provider) => format!("Unknown APIProvider: {provider}"),
                Self::ParseError { error, .. } =>
                    format!("Failed to JSON parse the response body: {error}"),
                Self::Non200StatusCode { status_code, .. } =>
                    format!("Received non-200 status code by Odesli: {status_code}"),
                Self::ReqwestError(error) => format!("Failed to make HTTP request: {error}"),
            }
        )
    }
}

impl std::error::Error for OdesliError {}
