use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use futures::{Stream, Future};
use futures::future::{FutureResult, Either, ok, empty};
use tk_http::client::{Proto, Config, Codec};
use tk_http::client::{Error, EncoderDone};
use tk_pool::{self, pool_for};
use tokio_core::reactor::{Handle, Timeout};
use tokio_core::net::TcpStream;

use pool_log::Logger;
use {Connection};

pub type Pool = tk_pool::queue::Pool<Box<Codec<TcpStream,
        Future=FutureResult<EncoderDone<TcpStream>, Error>>>,
        tk_pool::metrics::Noop>;

const LOCAL_CONNECT_TIMEOUT: Duration = Duration::from_secs(1);

/// Connects to the cantal instance on localhost
pub fn connect_local(h: &Handle) -> Connection {
    let h1 = h.clone();
    let connection_config = Config::new()
        .inflight_request_limit(2)
        .done();
    let pool = pool_for(move |addr| {
            Proto::connect_tcp(addr, &connection_config, &h1)
            .select2(Timeout::new(LOCAL_CONNECT_TIMEOUT, &h1)
                .expect("timeout never fails"))
            .then(|res| match res {
                Ok(Either::A((c, _))) => Ok(c),
                Ok(Either::B((_, _))) => {
                    warn!("Timeout connecting to local cantal");
                    Err(Error::custom(
                        io::Error::from(io::ErrorKind::TimedOut)))
                }
                Err(Either::A((e, _))) => Err(e),
                Err(Either::B((_, _))) => unreachable!(),
            })
        })
        .connect_to(
            ok("127.0.0.1:22682".parse::<SocketAddr>().unwrap().into())
                .into_stream()
                .chain(empty().into_stream()))
        .lazy_uniform_connections(1)
        .with_queue_size(1)
        .errors(Logger)
        .spawn_on(h);
    return Connection {
        pool: pool,
    }
}
