use failure::{Fail, Error, Context, Backtrace};
use std::fmt::{self, Display, Formatter};
use std::error::Error as StdError;
use futures::Future as StdFuture;
use futures::Async;
use std::result::Result as StdResult;
use std::ops::{Deref, DerefMut};

#[allow(dead_code)]
pub type Result<T, E=Error> = StdResult<T, E>;
#[allow(dead_code)]
pub type Future<T, E=Error> = StdFuture<Item=T, Error=E>;

#[allow(dead_code)]
pub struct FutureBox<T: 'static, E: 'static = Error>(Box<Future<T, E>>);

impl<T: 'static, E: 'static> StdFuture for FutureBox<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> StdResult<Async<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

impl<T: 'static, E: 'static> Deref for FutureBox<T, E> {
    type Target = StdFuture<Item=T, Error=E>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, E: 'static> DerefMut for FutureBox<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

