//! # Cantal Client
//!
//! [Documentation](https://docs.rs/tk-cantal) |
//! [Github](https://github.com/tailhook/tk-cantal) |
//! [Crate](https://crates.io/crates/tk-cantal)
//!
//! This client is usually used to find out peers known to cantal, i.e. peers
//! of the current cluster.
//!
//! This is **not** a way to submit metrics to cantal. See [`libcantal`] for
//! that.
//!
//! We will expose more APIs, like fetching metrics later.
//!
//! [`libcantal`]: https://crates.io/crates/libcantal
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
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

use std::fmt;

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

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Connection").finish()
    }
}
