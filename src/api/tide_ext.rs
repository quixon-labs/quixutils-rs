use http::Response as HttpResponse;
use http::StatusCode;
use http_service::Body;
use tide::{body::Json, IntoResponse, Response};

use super::{ApiError, ErrorData};

impl<T: ErrorData> IntoResponse for ApiError<T> {
    fn into_response(self) -> Response {
        match self {
            ApiError::UserError(d) => json(StatusCode::OK, d),
            ApiError::BadRequest(o) => {
                if let Some(d) = o {
                    json(StatusCode::BAD_REQUEST, d)
                } else {
                    empty(StatusCode::BAD_REQUEST)
                }
            }
            ApiError::UnprocessableEntity(o) => {
                if let Some(d) = o {
                    json(StatusCode::UNPROCESSABLE_ENTITY, d)
                } else {
                    empty(StatusCode::UNPROCESSABLE_ENTITY)
                }
            }
            ApiError::TooManyRequests {
                retry_after_secs: r,
            } => {
                if let Some(t) = r {
                    HttpResponse::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", t.to_string())
                        .body(Body::empty())
                        .unwrap()
                } else {
                    empty(StatusCode::TOO_MANY_REQUESTS)
                }
            }
            ApiError::Unauthorized => empty(StatusCode::UNAUTHORIZED),
            ApiError::Forbidden => empty(StatusCode::FORBIDDEN),
            ApiError::NotFound => empty(StatusCode::NOT_FOUND),
            ApiError::BadGateway => empty(StatusCode::BAD_GATEWAY),
            ApiError::GatewayTimeout => empty(StatusCode::GATEWAY_TIMEOUT),
            ApiError::Internal { error: e } => {
                log::error!("{:?}", e);
                empty(StatusCode::INTERNAL_SERVER_ERROR)
            }
            ApiError::Unknown => empty(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

fn empty(code: StatusCode) -> Response {
    HttpResponse::builder()
        .status(code)
        .body(Body::empty())
        .unwrap()
}

fn json<T: Send + serde::Serialize>(code: StatusCode, body: T) -> Response {
    let mut response = Json(body).into_response();
    *response.status_mut() = code;
    response
}
