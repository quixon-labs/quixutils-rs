use failure::{Backtrace, Context, Error, Fail};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display, Formatter};
use std::error::Error as StdError;
use std::ops::{Deref, DerefMut, Try};

type DefaultApiErrorData = ErrorItems;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ApiResult<T: Serialize, E: ErrorData = DefaultApiErrorData> {
    Ok(T),
    Err(ApiError<E>),
}

impl<T: Serialize, E: ErrorData> From<ApiError<E>> for ApiResult<T, E> {
    fn from(e: ApiError<E>) -> Self {
        ApiResult::Err(e)
    }
}

impl<T: Serialize, E: ErrorData> Try for ApiResult<T, E> {
    type Ok = T;
    type Error = ApiError<E>;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            ApiResult::Ok(v) => Ok(v),
            ApiResult::Err(e) => Err(e),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        ApiResult::Err(v)
    }

    fn from_ok(v: Self::Ok) -> Self {
        ApiResult::Ok(v)
    }
}

impl<T: Serialize, E: ErrorData> ApiResult<T, E> {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        match *self {
            ApiResult::Ok(_) => true,
            ApiResult::Err(_) => false
        }
    }

    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[allow(dead_code)]
    pub fn ok(self) -> Option<T> {
        match self {
            ApiResult::Ok(x)  => Some(x),
            ApiResult::Err(_) => None,
        }
    }

    #[allow(dead_code)]
    pub fn err(self) -> Option<ApiError<E>> {
        match self {
            ApiResult::Ok(_)  => None,
            ApiResult::Err(x) => Some(x),
        }
    }

    #[allow(dead_code)]
    pub fn map<U: Serialize, F: FnOnce(T) -> U>(self, op: F) -> ApiResult<U,E> {
        match self {
            ApiResult::Ok(t) => ApiResult::Ok(op(t)),
            ApiResult::Err(e) => ApiResult::Err(e)
        }
    }

    #[allow(dead_code)]
    pub fn map_or_else<U: Serialize, M: FnOnce(T) -> U, F: FnOnce(ApiError<E>) -> U>(self, fallback: F, map: M) -> U {
        self.map(map).unwrap_or_else(fallback)
    }

    #[allow(dead_code)]
    pub fn map_err<F: ErrorData, O: FnOnce(ApiError<E>) -> ApiError<F>>(self, op: O) -> ApiResult<T,F> {
        match self {
            ApiResult::Ok(t) => ApiResult::Ok(t),
            ApiResult::Err(e) => ApiResult::Err(op(e))
        }
    }

    #[allow(dead_code)]
    pub fn and<U: Serialize>(self, res: ApiResult<U, E>) -> ApiResult<U, E> {
        match self {
            ApiResult::Ok(_) => res,
            ApiResult::Err(e) => ApiResult::Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn and_then<U: Serialize, F: FnOnce(T) -> ApiResult<U, E>>(self, op: F) -> ApiResult<U, E> {
        match self {
            ApiResult::Ok(t) => op(t),
            ApiResult::Err(e) => ApiResult::Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn or<F: ErrorData>(self, res: ApiResult<T, F>) -> ApiResult<T, F> {
        match self {
            ApiResult::Ok(v) => ApiResult::Ok(v),
            ApiResult::Err(_) => res,
        }
    }

    #[allow(dead_code)]
    pub fn or_else<F: ErrorData, O: FnOnce(ApiError<E>) -> ApiResult<T, F>>(self, op: O) -> ApiResult<T, F> {
        match self {
            ApiResult::Ok(t) => ApiResult::Ok(t),
            ApiResult::Err(e) => op(e),
        }
    }

    #[allow(dead_code)]
    pub fn unwrap_or(self, optb: T) -> T {
        match self {
            ApiResult::Ok(t) => t,
            ApiResult::Err(_) => optb
        }
    }

    #[allow(dead_code)]
    pub fn unwrap_or_else<F: FnOnce(ApiError<E>) -> T>(self, op: F) -> T {
        match self {
            ApiResult::Ok(t) => t,
            ApiResult::Err(e) => op(e)
        }
    }

    #[allow(dead_code)]
    pub fn into_result(self) -> Result<T, ApiError<E>> {
        match self {
            ApiResult::Ok(x) => Result::Ok(x),
            ApiResult::Err(x) => Result::Err(x),
        }
    }
}

pub trait ErrorData: Serialize + Send + Sync + Debug + 'static {}
impl ErrorData for () {}

// For a small set of values, B-Tree map is usually more efficient
// than a hash-map.
pub type ErrorItems = BTreeMap<String, String>;
impl ErrorData for ErrorItems {}

#[derive(Debug, Serialize, Fail)]
pub enum ApiError<D: ErrorData = ()> {
    #[allow(dead_code)]
    UserError(D),
    #[allow(dead_code)]
    BadRequest(#[serde(skip_serializing_if = "Option::is_none")] Option<D>),
    #[allow(dead_code)]
    UnprocessableEntity(#[serde(skip_serializing_if = "Option::is_none")] Option<D>),
    #[allow(dead_code)]
    TooManyRequests {
        #[serde(skip_serializing_if = "Option::is_none")]
        retry_after_secs: Option<i32>,
    },
    #[allow(dead_code)]
    Unauthorized,
    #[allow(dead_code)]
    Forbidden,
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    BadGateway,
    #[allow(dead_code)]
    GatewayTimeout,
    #[allow(dead_code)]
    Internal { #[serde(skip)] error: Error },
    #[allow(dead_code)]
    Unknown,
}

impl<D: ErrorData> From<Error> for ApiError<D> {
    fn from(error: Error) -> Self {
        ApiError::Internal { error }
    }
}

impl<D: ErrorData> Display for ApiError<D> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ApiError::UserError(d) => {
                writeln!(f, "user error: {:?}", d)
            },
            ApiError::BadRequest(o) => {
                if let Some(d) = o {
                    writeln!(f, "error: bad request: {:?}", d)
                } else {
                    writeln!(f, "error: bad request")
                }
            },
            ApiError::UnprocessableEntity(o) => {
                if let Some(d) = o {
                    writeln!(f, "error: unprocessable entity: {:?}", d)
                } else {
                    writeln!(f, "error: unprocessable entity")
                }
            },
            ApiError::TooManyRequests { retry_after_secs: r } => {
                if let Some(t) = r {
                    writeln!(f, "error: too many requests - retry_in: {:?}", t)
                } else {
                    writeln!(f, "error: too many requests")
                }
            },
            ApiError::Unauthorized => {
                writeln!(f, "error: unauthorized")
            },
            ApiError::Forbidden => {
                writeln!(f, "error: forbidden")
            },
            ApiError::NotFound => {
                writeln!(f, "error: not found")
            },
            ApiError::BadGateway => {
                writeln!(f, "error: bad gateway")
            },
            ApiError::GatewayTimeout => {
                writeln!(f, "error: gateway timeout")
            },
            ApiError::Internal { error: e } => {
                writeln!(f, "error: {:?}", e)
            },
            ApiError::Unknown => {
                writeln!(f, "unknown error")
            }
        }
    }
}

