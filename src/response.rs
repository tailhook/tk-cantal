use std::marker::PhantomData;

use futures::{Future, Async};
use futures::sync::oneshot::Receiver;
use tk_http::client::Error;


pub struct ResponseFuture<T> {
    phantom: PhantomData<T>,
}

impl<T> Future for ResponseFuture<T> {
    type Item = T;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<T>, Error> {
        unimplemented!();
    }
}

pub fn not_connected<T>() -> ResponseFuture<T> {
    unimplemented!();
}

pub fn from_channel<T>(s: Receiver<T>) -> ResponseFuture<T> {
    unimplemented!();
}

pub fn from_error<T, E>(e: Error) -> ResponseFuture<T> {
    unimplemented!();
}
