use std::net::SocketAddr;

use tk_http::client::Error;
use tk_pool::error_log::{ErrorLog, ShutdownReason};
use tk_pool::config::NewErrorLog;


#[derive(Clone)]
pub struct Logger;


impl NewErrorLog<Error, Error> for Logger {
    type ErrorLog = Logger;
    fn construct(self) -> Self::ErrorLog {
        Logger
    }
}

impl ErrorLog for Logger {
    type ConnectionError = Error;
    type SinkError = Error;
    fn connection_error(&self, addr: SocketAddr, e: Error) {
        warn!("Connecting to {} failed: {}", addr, e);
    }
    fn sink_error(&self, addr: SocketAddr, e: Error) {
        if e.is_graceful() {
            debug!("Connection to {} errored: {}", addr, e);
        } else {
            warn!("Connection to {} errored: {}", addr, e);
        }
    }
    /// Starting to shut down pool
    fn pool_shutting_down(&self, reason: ShutdownReason) {
        warn!("Shutting down connection pool: {}", reason);
    }
    /// This is triggered when pool done all the work and shut down entirely
    fn pool_closed(&self) {
    }
}
