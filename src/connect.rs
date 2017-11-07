use std::net::SocketAddr;

use futures::{Stream, Future};
use futures::future::{FutureResult, ok, empty};
use tk_http::client::{Proto, Config as HConfig, Codec};
use tk_http::client::{Error, EncoderDone};
use tk_pool::uniform::{UniformMx, Config as PConfig};
use tk_pool;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;

use {Connection};

pub type Pool = tk_pool::Pool<Box<Codec<TcpStream,
        Future=FutureResult<EncoderDone<TcpStream>, Error>>>>;

/// Connects to the cantal instance on localhost
pub fn connect_local(h: &Handle) -> Connection {
    let h1 = h.clone();
    let pool_config = PConfig::new()
        .connections_per_address(1)
        .done();
    let connection_config = HConfig::new()
        .inflight_request_limit(2)
        .done();
    let multiplexer = UniformMx::new(&h,
        &pool_config,
        ok("127.0.0.1:22682".parse::<SocketAddr>().unwrap().into())
            .into_stream()
            .chain(empty::<_, Error>().into_stream()),
        move |addr| Proto::connect_tcp(addr, &connection_config, &h1));
    let queue_size = 10;
    let pool = tk_pool::Pool::create(&h, queue_size, multiplexer);
    return Connection {
        pool: pool,
    }
}
