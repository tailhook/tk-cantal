use std::time::SystemTime;

use futures::{Async, Sink, AsyncSink};
use futures::future::{FutureResult, ok};
use futures::sync::oneshot::{channel, Sender, Receiver};
use tk_http::client as http;
use tk_http::{Status, Version};
use tokio_core::io::Io;

use {Connection, ResponseFuture};
use errors::BadResponse;
use response;


/// Info about the peer
///
/// We currently include only a subset of data reported by cantal here.
/// Mostly things that are unlikely to change in future. This will be fixed
/// when cantal grows stable API.
// TODO(tailhook) Turn timestamps into timestamps.
//                We keep old values here for easier migration of verwalter,
//                should fix as soon as possible.
// TODO(tailhook) Use serde
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Peer {
    pub id: String,
    pub hostname: String,
    pub name: String,
    pub primary_addr: Option<String>,
    pub addresses: Vec<String>,
    /// Known to this host, unixtime in milliseconds
    pub known_since: u64,
    /// Last report directly to this node unixtime in milliseconds
    pub last_report_direct: Option<u64>,
}

#[derive(Debug)]
pub struct PeersResponse {
    pub timestamp: SystemTime,
    pub received: SystemTime,
    pub peers: Vec<Peer>,
}


struct PeersCodec {
    tx: Sender<PeersResponse>,
}

impl Connection {
    pub fn get_peers(&self) -> ResponseFuture<PeersResponse> {
        let (tx, rx) = channel();
        match self.pool.clone().start_send(Box::new(PeersCodec { tx: tx })) {
            Ok(AsyncSink::NotReady(_)) => response::not_connected(),
            Ok(AsyncSink::Ready) => response::from_channel(rx),
            Err(_send_error) => response::not_connected(),
        }
    }
}

impl<S: Io> http::Codec<S> for PeersCodec {
    type Future = FutureResult<http::EncoderDone<S>, http::Error>;

    fn start_write(&mut self, mut e: http::Encoder<S>) -> Self::Future {
        e.request_line("GET", "/all_peers.json", Version::Http11);
        e.done_headers().unwrap();
        ok(e.done())
    }
    fn headers_received(&mut self, headers: &http::Head)
        -> Result<http::RecvMode, http::Error>
    {
        if headers.status() != Some(Status::Ok) {
            return Err(http::Error::custom(
                BadResponse::Status(headers.status())));
        }
        Ok(http::RecvMode::buffered(10_485_760))
    }
    fn data_received(&mut self, data: &[u8], end: bool)
        -> Result<Async<usize>, http::Error>
    {
        unimplemented!();
    }
}
