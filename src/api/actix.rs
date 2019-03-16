use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use actix_web::dev::AsyncResult;
use actix_web::http::StatusCode;
use actix_web::error::Error as ActixError;

impl <D: ErrorData> From<ActixError> for ApiError<D> {
    fn from(error: ActixError) -> Self {
        let f = format_err!("{:?}", error);
        ApiError::Internal { error: f }
    }
}

impl <D: ErrorData> From<::actix::MailboxError> for ApiError<D> {
    fn from(error: ::actix::MailboxError) -> Self {
        let f = format_err!("{:?}", error);
        ApiError::Internal { error: f }
    }
}

impl<D: ErrorData> ResponseError for ApiError<D> {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::UserError(d) => {
                HttpResponse::Ok()
                    .json(d)
            },
            ApiError::BadRequest(o) => {
                if let Some(d) = o {
                    HttpResponse::build(StatusCode::BAD_REQUEST)
                        .json(d)
                } else {
                    HttpResponse::new(StatusCode::BAD_REQUEST)
                }
            },
            ApiError::UnprocessableEntity(o) => {
                if let Some(d) = o {
                    HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY)
                        .json(d)
                } else {
                    HttpResponse::new(StatusCode::UNPROCESSABLE_ENTITY)
                }
            },
            ApiError::TooManyRequests { retry_after_secs: r } => {
                if let Some(t) = r {
                    HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", t.to_string())
                        .finish()
                } else {
                    HttpResponse::new(StatusCode::TOO_MANY_REQUESTS)
                }
            },
            ApiError::Unauthorized => {
                HttpResponse::new(StatusCode::UNAUTHORIZED)
            },
            ApiError::Forbidden => {
                HttpResponse::new(StatusCode::FORBIDDEN)
            },
            ApiError::NotFound => {
                HttpResponse::new(StatusCode::NOT_FOUND)
            },
            ApiError::BadGateway => {
                HttpResponse::new(StatusCode::BAD_GATEWAY)
            },
            ApiError::GatewayTimeout => {
                HttpResponse::new(StatusCode::GATEWAY_TIMEOUT)
            },
            ApiError::Internal { error: e } => {
                error!("{:?}", e);
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            },
            ApiError::Unknown => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Responder impl

impl<T, E> Responder for ApiResult<T, E>
    where
        T: Serialize,
        E: ErrorData,
{
    type Item = HttpResponse;
    type Error = ApiError<E>;

    fn respond_to<S: 'static>(
        self,
        _req: &HttpRequest<S>,
    ) -> std::result::Result<Self::Item, Self::Error> {
        let res = match self {
            ApiResult::Ok(v) => {
                let payload: ApiResult<T, E> = ApiResult::Ok(v);
                Ok(HttpResponse::Ok().json(payload))
            }
            ApiResult::Err(e) => Err(e),
        };
        res
    }
}