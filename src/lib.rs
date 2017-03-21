extern crate abstract_ns;
extern crate futures;
extern crate rustc_serialize;
extern crate tk_http;
extern crate tk_pool;
extern crate tokio_core;

#[macro_use] extern crate quick_error;

mod connect;
mod peers;
mod response;
mod errors;

pub use connect::connect_local;
pub use response::ResponseFuture;


pub struct Connection {
    pool: connect::Pool,
}
