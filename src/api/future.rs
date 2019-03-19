use super::{ApiError, ApiResult, ErrorData, ErrorItems};
use futures::future::ok;
use futures::{Async, Future as StdFuture};
use serde::Serialize;
use std::ops::{Deref, DerefMut};

pub struct ApiFuture<T: Serialize + 'static, D: ErrorData = ErrorItems>(
    Box<StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>>>,
);

impl<T: Serialize, D: ErrorData> ApiFuture<T, D> {
    pub fn new<F: StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>> + 'static>(
        future: F,
    ) -> Self {
        ApiFuture(Box::new(future))
    }

    pub fn err<E: Into<ApiError<D>>>(error: E) -> Self {
        let res = ApiResult::Err(error.into());
        ApiFuture::result(res)
    }

    pub fn ok(data: T) -> Self {
        let res = ApiResult::Ok(data);
        ApiFuture::result(res)
    }

    pub fn result(r: ApiResult<T, D>) -> Self {
        ApiFuture::new(ok(r))
    }

    pub fn from_boxed<F: StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>> + 'static>(
        f: Box<F>,
    ) -> Self {
        ApiFuture(f as Box<StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>>>)
    }
}

impl<T: Serialize, D: ErrorData> From<ApiResult<T, D>> for ApiFuture<T, D> {
    fn from(r: ApiResult<T, D>) -> Self {
        ApiFuture::result(r)
    }
}

impl<T: Serialize, D: ErrorData, F> From<Box<F>> for ApiFuture<T, D>
where
    F: StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>> + 'static,
{
    fn from(f: Box<F>) -> Self {
        ApiFuture::from_boxed(f)
    }
}

impl<T: Serialize + 'static, D: ErrorData> Deref for ApiFuture<T, D> {
    type Target = StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + 'static, D: ErrorData> DerefMut for ApiFuture<T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize + 'static, D: ErrorData> StdFuture for ApiFuture<T, D> {
    type Item = ApiResult<T, D>;
    type Error = ApiError<D>;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.0.poll()
    }
}
