use std::marker::PhantomData;

use futures::{Future, Async};
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
