use std::net::SocketAddr;

use futures::{Stream, Future};
use futures::future::{FutureResult, ok, empty};
use tk_http::client::{Proto, Config, Codec};
use tk_http::client::{Error, EncoderDone};
use tk_pool::{self, pool_for};
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;

use {Connection};

pub type Pool = tk_pool::queue::Pool<Box<Codec<TcpStream,
        Future=FutureResult<EncoderDone<TcpStream>, Error>>>,
        tk_pool::metrics::Noop>;

/// Connects to the cantal instance on localhost
pub fn connect_local(h: &Handle) -> Connection {
    let h1 = h.clone();
    let connection_config = Config::new()
        .inflight_request_limit(2)
        .done();
    let pool = pool_for(move |addr| {
            Proto::connect_tcp(addr, &connection_config, &h1)
        })
        .connect_to(
            ok("127.0.0.1:22682".parse::<SocketAddr>().unwrap().into())
                .into_stream()
                .chain(empty().into_stream()))
        .lazy_uniform_connections(1)
        .with_queue_size(1)
        .spawn_on(h);
    return Connection {
        pool: pool,
    }
}
