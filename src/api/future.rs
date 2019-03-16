use futures::future::{err, ok, result};
use futures::{Async, Future as StdFuture};
use serde::{Deserialize, Serialize};

pub struct ApiFuture<T: Serialize + 'static, D: ErrorData = DefaultApiErrorData>(
    Box<StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>>>,
);

impl<T: Serialize, D: ErrorData> ApiFuture<T, D> {
    #[allow(dead_code)]
    pub fn new<F: StdFuture<Item = ApiResult<T, D>, Error = ApiError<D>> + 'static>(
        future: F,
    ) -> Self {
        ApiFuture(Box::new(future))
    }

    #[allow(dead_code)]
    pub fn err<E: Into<ApiError<D>>>(error: E) -> Self {
        let res = ApiResult::Err(error.into());
        ApiFuture::result(res)
    }

    #[allow(dead_code)]
    pub fn ok(data: T) -> Self {
        let res = ApiResult::Ok(data);
        ApiFuture::result(res)
    }

    #[allow(dead_code)]
    pub fn result(r: ApiResult<T, D>) -> Self {
        ApiFuture::new(ok(r))
    }

    #[allow(dead_code)]
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
