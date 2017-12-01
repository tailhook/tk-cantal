use failure::{Fail};
use futures::{Future, Async};
use futures::sync::oneshot::Receiver;
use tk_http::client::Error;
use errors::BadResponse;


/// Future returned by connection methods that fetches data T from cantal
#[derive(Debug)]
pub struct ResponseFuture<T>(State<T>);

#[derive(Debug)]
pub(crate) enum State<T> {
    Waiting(Receiver<T>),
    Error(Option<Error>),
}

impl<T> Future for ResponseFuture<T> {
    type Item = T;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<T>, Error> {
        match self.0 {
            State::Waiting(ref mut f) => match f.poll() {
                Ok(x) => Ok(x),
                Err(_) => Err(Error::custom(BadResponse::Canceled.compat())),
            },
            State::Error(ref mut e) => {
                Err(e.take().expect("error is not taken"))
            }
        }
    }
}

pub fn not_connected<T>() -> ResponseFuture<T> {
    ResponseFuture(State::Error(Some(
        Error::custom(BadResponse::NotConnected.compat()))))
}

pub fn from_channel<T>(s: Receiver<T>) -> ResponseFuture<T> {
    ResponseFuture(State::Waiting(s))
}
