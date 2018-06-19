use http::header::InvalidHeaderValue;
use {bytes, http, hyper, hyper_tls, serde_json};

/// Error that can occur when creating a client.
#[derive(Fail, Debug)]
pub enum StartupError {
    /// Error starting TLS connector
    #[fail(display = "tls error: {}", _0)]
    Tls(hyper_tls::Error),
    /// Token provided was invalid
    #[fail(display = "invalid token: {} (token: {:?})", _0, _1)]
    InvalidToken(InvalidHeaderValue, String),
}

/// Error that can occur when running a request.
#[derive(Fail, Debug)]
pub enum RequestError {
    /// Error creating request.
    #[fail(display = "request error: {}", _0)]
    Http(http::Error),
    /// Hyper error.
    #[fail(display = "hyper error: {}", _0)]
    Hyper(hyper::Error),
    /// Generic server error.
    #[fail(display = "server error: {}: {:?}", status, bytes)]
    Status {
        /// The failed server status.
        status: http::StatusCode,
        /// The response included with this status.
        bytes: bytes::Bytes,
    },
    /// Invalid JSON in response.
    #[fail(display = "invalid response json: {}. data: {:?}", error, bytes)]
    Json {
        /// Inner error
        error: serde_json::Error,
        /// Bytes which could not be decoded
        bytes: bytes::Bytes,
    },
    /// Server error.
    #[fail(display = "server error: {}: {}", code, message)]
    Server {
        /// Error code string
        code: String,
        /// Human readable error message
        message: String,
    },
}

impl From<hyper::Error> for RequestError {
    fn from(e: hyper::Error) -> Self {
        RequestError::Hyper(e)
    }
}

impl From<http::Error> for RequestError {
    fn from(e: http::Error) -> Self {
        RequestError::Http(e)
    }
}
