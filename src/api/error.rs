use failure::{Error, Fail};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display, Formatter};

pub trait ErrorData: Serialize + Send + Sync + Debug + 'static {}
impl ErrorData for () {}

// For a small set of values, B-Tree map is usually more efficient
// than a hash-map.
pub type ErrorItems = BTreeMap<String, String>;
impl ErrorData for ErrorItems {}

#[derive(Debug, Serialize, Fail)]
pub enum ApiError<D: ErrorData = ()> {
    UserError(D),
    BadRequest(#[serde(skip_serializing_if = "Option::is_none")] Option<D>),
    UnprocessableEntity(#[serde(skip_serializing_if = "Option::is_none")] Option<D>),
    TooManyRequests {
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_after_secs: Option<i32>,
    },
    Unauthorized,
    Forbidden,
    NotFound,
    BadGateway,
    GatewayTimeout,
    Internal {
        #[serde(skip)]
        error: Error,
    },
    Unknown,
}

impl<D: ErrorData> From<Error> for ApiError<D> {
    fn from(error: Error) -> Self {
        ApiError::Internal { error }
    }
}

impl<D: ErrorData> From<std::io::Error> for ApiError<D> {
    fn from(error: std::io::Error) -> Self {
        ApiError::Internal {
            error: failure::Error::from(error),
        }
    }
}

impl<D: ErrorData> Display for ApiError<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::ApiError::*;
        match self {
            UserError(d) => writeln!(f, "user error: {:?}", d),
            BadRequest(o) => {
                if let Some(d) = o {
                    writeln!(f, "error: bad request: {:?}", d)
                } else {
                    writeln!(f, "error: bad request")
                }
            }
            UnprocessableEntity(o) => {
                if let Some(d) = o {
                    writeln!(f, "error: unprocessable entity: {:?}", d)
                } else {
                    writeln!(f, "error: unprocessable entity")
                }
            }
            TooManyRequests {
                retry_after_secs: r,
            } => {
                if let Some(t) = r {
                    writeln!(f, "error: too many requests - retry_in: {:?}", t)
                } else {
                    writeln!(f, "error: too many requests")
                }
            }
            Unauthorized => writeln!(f, "error: unauthorized"),
            Forbidden => writeln!(f, "error: forbidden"),
            NotFound => writeln!(f, "error: not found"),
            BadGateway => writeln!(f, "error: bad gateway"),
            GatewayTimeout => writeln!(f, "error: gateway timeout"),
            Internal { error: e } => writeln!(f, "error: {:?}", e),
            Unknown => writeln!(f, "unknown error"),
        }
    }
}
