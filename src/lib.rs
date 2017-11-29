#[warn(missing_docs)]
extern crate abstract_ns;
extern crate failure;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate serde_millis;
extern crate tk_http;
extern crate tk_pool;
extern crate tokio_core;
extern crate tokio_io;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate failure_derive;

mod connect;
mod peers;
mod response;
mod errors;

pub use connect::connect_local;
pub use response::ResponseFuture;
pub use peers::{PeersResponse, Peer};


/// Connection abstraction used to fetch data from the cantal
///
/// Internally this structure contains a connection pool that reconnects
/// when connection is broken.
pub struct Connection {
    pool: connect::Pool,
}
