use super::{ApiError, ErrorData};
use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

// Errors impl

impl<D: ErrorData> ResponseError for ApiError<D> {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::UserError(d) => HttpResponse::Ok().json(d),
            ApiError::BadRequest(o) => {
                if let Some(d) = o {
                    HttpResponse::build(StatusCode::BAD_REQUEST).json(d)
                } else {
                    HttpResponse::new(StatusCode::BAD_REQUEST)
                }
            }
            ApiError::UnprocessableEntity(o) => {
                if let Some(d) = o {
                    HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(d)
                } else {
                    HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY)
                }
            }
            ApiError::TooManyRequests {
                retry_after_secs: r,
            } => {
                if let Some(t) = r {
                    HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", t.to_string())
                        .finish()
                } else {
                    HttpResponse::new(StatusCode::TOO_MANY_REQUESTS)
                }
            }
            ApiError::Unauthorized => HttpResponse::new(StatusCode::UNAUTHORIZED),
            ApiError::Forbidden => HttpResponse::new(StatusCode::FORBIDDEN),
            ApiError::NotFound => HttpResponse::new(StatusCode::NOT_FOUND),
            ApiError::BadGateway => HttpResponse::new(StatusCode::BAD_GATEWAY),
            ApiError::GatewayTimeout => HttpResponse::new(StatusCode::GATEWAY_TIMEOUT),
            ApiError::Internal { error: e } => {
                log::error!("{:?}", e);
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            ApiError::Unknown => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
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
