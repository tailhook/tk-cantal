use tk_http::Status;

#[derive(Debug, Fail)]
pub enum BadResponse {
    #[fail(display="http response has status {:?} (200 required)", _0)]
    Status(Option<Status>),
    #[fail(display="request was canceled")]
    Canceled,
    #[fail(display="connection is unavailable yet")]
    NotConnected,
}
