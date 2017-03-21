use tk_http::Status;

quick_error! {
    #[derive(Debug)]
    pub enum BadResponse {
        Status(s: Option<Status>) {
            description("http response has non-200 status")
            display("http response has status {:?} (200 required)", s)
        }
    }
}
