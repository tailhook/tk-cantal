use std::time::SystemTime;

use failure::{Fail};
use futures::{Async, Sink, AsyncSink};
use futures::future::{FutureResult, ok};
use futures::sync::oneshot::{channel, Sender};
use serde_json::from_slice;
use tk_http::client as http;
use tk_http::{Status, Version};

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
#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub hostname: String,
    pub name: String,
    pub primary_addr: Option<String>,
    pub addresses: Vec<String>,
    /// Known to this host, unixtime in milliseconds
    #[serde(with="::serde_millis")]
    pub known_since: SystemTime,
    // /// Last report directly to this node unixtime in milliseconds
    // TODO(tailhook) when serde_millis fixes it
    //#[serde(with="::serde_millis")]
    //pub last_report_direct: Option<SystemTime>,
}

/// A response to the `get_peers()` request
#[derive(Debug)]
pub struct PeersResponse {
    /// A timestamp when `get_peers()` wasa issued
    pub requested: SystemTime,
    /// A timestamp when response to the request was received
    pub received: SystemTime,
    /// Actual list of peer data
    pub peers: Vec<Peer>,
}


struct PeersCodec {
    request_time: SystemTime,
    tx: Option<Sender<PeersResponse>>,
}

impl Connection {
    pub fn get_peers(&self) -> ResponseFuture<PeersResponse> {
        let (tx, rx) = channel();
        let pcodec = PeersCodec {
            request_time: SystemTime::now(),
            tx: Some(tx),
        };
        match self.pool.clone().start_send(Box::new(pcodec)) {
            Ok(AsyncSink::NotReady(_)) => response::not_connected(),
            Ok(AsyncSink::Ready) => response::from_channel(rx),
            Err(_send_error) => response::not_connected(),
        }
    }
}

impl<S> http::Codec<S> for PeersCodec {
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
                BadResponse::Status(headers.status()).compat()));
        }
        Ok(http::RecvMode::buffered(10_485_760))
    }
    fn data_received(&mut self, data: &[u8], end: bool)
        -> Result<Async<usize>, http::Error>
    {
        #[derive(Debug, Deserialize, Serialize)]
        pub struct Response {
            peers: Vec<Peer>,
        }

        debug_assert!(end);
        let decoded: Response = match from_slice(data) {
            Ok(data) => data,
            Err(e) => {
                error!("Error decoding peers data: {}", e);
                drop(self.tx.take().expect("sender is still alive"));
                return Ok(Async::Ready(data.len()));
            }
        };
        self.tx.take().expect("sender is still alive").send(PeersResponse {
            requested: self.request_time,
            received: SystemTime::now(),
            peers: decoded.peers,
        }).map_err(|_| {
            debug!("Can't send response for peers request, oneshot is closed");
        }).ok();
        return Ok(Async::Ready(data.len()));
    }
}
