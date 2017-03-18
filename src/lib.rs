extern crate abstract_ns;
extern crate futures;
extern crate tk_http;
extern crate tk_pool;
extern crate tokio_core;

mod connect;
mod peers;
mod response;

pub use connect::connect_local;
pub use response::ResponseFuture;


pub struct Connection {
    pool: connect::Pool,
}
