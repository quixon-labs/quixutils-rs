use failure::Error;
use futures::Async;
use futures::Future;
use std::ops::{Deref, DerefMut};
use std::result::Result;

pub type ResultAs<T, E = Error> = Result<T, E>;

// TODO: Switch to trait aliases after
// bug with it are resolved: https://github.com/rust-lang/rust/issues/41517
//
// pub trait FutureAs<T, E = Error> = Future<Item = T, Error = E>;

pub struct FutureBox<T: 'static, E: 'static = Error>(Box<Future<Item = T, Error = E>>);

impl<T: 'static, E: 'static> Future for FutureBox<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

impl<T: 'static, E: 'static> Deref for FutureBox<T, E> {
    type Target = Future<Item = T, Error = E>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static, E: 'static> DerefMut for FutureBox<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
