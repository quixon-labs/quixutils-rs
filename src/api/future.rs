use super::{ApiError, ApiResult, ErrorData, ErrorItems};
use futures::future::ok;
use futures::{Async, Future};
use serde::Serialize;
use std::ops::{Deref, DerefMut};

pub trait ApiFuture<T: Serialize + 'static, D: ErrorData = ErrorItems> =
    Future<Item = ApiResult<T, D>, Error = ApiError<D>>;

pub struct ApiFutureBox<T: Serialize + 'static, D: ErrorData = ErrorItems>(Box<ApiFuture<T, D>>);

impl<T: Serialize, D: ErrorData> ApiFutureBox<T, D> {
    pub fn new<F: Future<Item = ApiResult<T, D>, Error = ApiError<D>> + 'static>(
        future: F,
    ) -> Self {
        ApiFutureBox(Box::new(future))
    }

    pub fn err<E: Into<ApiError<D>>>(error: E) -> Self {
        let res = ApiResult::Err(error.into());
        ApiFutureBox::result(res)
    }

    pub fn ok(data: T) -> Self {
        let res = ApiResult::Ok(data);
        ApiFutureBox::result(res)
    }

    pub fn result(r: ApiResult<T, D>) -> Self {
        ApiFutureBox::new(ok(r))
    }

    pub fn from_boxed<F: ApiFuture<T, D> + 'static>(f: Box<F>) -> Self {
        ApiFutureBox(f)
    }

    pub fn into_inner(self) -> Box<ApiFuture<T, D>> {
        self.0
    }
}

impl<T: Serialize, D: ErrorData> From<ApiResult<T, D>> for ApiFutureBox<T, D> {
    fn from(r: ApiResult<T, D>) -> Self {
        ApiFutureBox::result(r)
    }
}

impl<T: Serialize, D: ErrorData, F> From<Box<F>> for ApiFutureBox<T, D>
where
    F: Future<Item = ApiResult<T, D>, Error = ApiError<D>> + 'static,
{
    fn from(f: Box<F>) -> Self {
        ApiFutureBox::from_boxed(f)
    }
}

impl<T: Serialize + 'static, D: ErrorData> Deref for ApiFutureBox<T, D> {
    type Target = Future<Item = ApiResult<T, D>, Error = ApiError<D>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + 'static, D: ErrorData> DerefMut for ApiFutureBox<T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Serialize + 'static, D: ErrorData> Future for ApiFutureBox<T, D> {
    type Item = ApiResult<T, D>;
    type Error = ApiError<D>;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.0.poll()
    }
}
