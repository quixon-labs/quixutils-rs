use http::Response as HttpResponse;
use http::StatusCode;
use http_service::Body;
use tide::{Response, response::IntoResponse};

use super::{ApiError, ErrorData};

impl<T: ErrorData> IntoResponse for ApiError<T> {
    fn into_response(self) -> Response {
        use self::ApiError::*;
        match self {
            UserError(d) => json(StatusCode::OK, d),
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
                    HttpResponse::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", t.to_string())
                        .body(Body::empty())
                        .unwrap()
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

fn empty(code: StatusCode) -> Response {
    HttpResponse::builder()
        .status(code)
        .body(Body::empty())
        .unwrap()
}

fn json<T: Send + serde::Serialize>(code: StatusCode, body: T) -> Response {
    let mut response = tide::response::json(body);
    *response.status_mut() = code;
    response
}
