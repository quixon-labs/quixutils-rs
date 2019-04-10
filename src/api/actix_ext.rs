use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use super::{ApiError, ErrorData};
use serde::Serialize;

// Errors impl

impl<D: ErrorData> ResponseError for ApiError<D> {
    fn error_response(&self) -> HttpResponse {
        use self::ApiError::*;
        match self {
            UserError(d) => HttpResponse::Ok().json(d),
            BadRequest(o) => {
                if let Some(d) = o {
                    json(StatusCode::BAD_REQUEST, d)
                } else {
                    empty(StatusCode::BAD_REQUEST)
                }
            }
            UnprocessableEntity(o) => {
                if let Some(d) = o {
                    json(StatusCode::UNPROCESSABLE_ENTITY, d)
                } else {
                    empty(StatusCode::UNPROCESSABLE_ENTITY)
                }
            }
            TooManyRequests {
                retry_after_secs: r,
            } => {
                if let Some(t) = r {
                    HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", t.to_string())
                        .finish()
                } else {
                    empty(StatusCode::TOO_MANY_REQUESTS)
                }
            }
            Unauthorized => empty(StatusCode::UNAUTHORIZED),
            Forbidden => empty(StatusCode::FORBIDDEN),
            NotFound => empty(StatusCode::NOT_FOUND),
            BadGateway => empty(StatusCode::BAD_GATEWAY),
            GatewayTimeout => empty(StatusCode::GATEWAY_TIMEOUT),
            Internal { error: e } => {
                log::error!("{:?}", e);
                empty(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Unknown => empty(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

fn empty(code: StatusCode) -> HttpResponse {
    HttpResponse::new(code)
}

fn json<T: Serialize>(code: StatusCode, body: T) -> HttpResponse {
    HttpResponse::build(code).json(body)
}

// From errors

impl<D: ErrorData> From<ActixError> for ApiError<D> {
    fn from(error: ActixError) -> Self {
        let f = failure::format_err!("{:?}", error);
        ApiError::Internal { error: f }
    }
}

impl<D: ErrorData> From<actix::MailboxError> for ApiError<D> {
    fn from(error: ::actix::MailboxError) -> Self {
        let f = failure::format_err!("{:?}", error);
        ApiError::Internal { error: f }
    }
}

impl<D: ErrorData> From<actix_web::error::JsonPayloadError> for ApiError<D> {
    fn from(error: ::actix_web::error::JsonPayloadError) -> ApiError<D> {
        let f = failure::format_err!("{:?}", error);
        ApiError::Internal { error: f }
    }
}
