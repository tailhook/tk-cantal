use std::env;
use std::time::Duration;

extern crate futures;
extern crate tk_easyloop;
extern crate tk_cantal;
extern crate env_logger;

use futures::{Stream, Future};
use tk_cantal::connect_local;

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "warn");
    }
    env_logger::init();
    tk_easyloop::run(|| {
        let conn = connect_local(&tk_easyloop::handle());
        tk_easyloop::interval(Duration::new(10, 0))
        .map_err(|_| unreachable!())
        .for_each(move |_| {
            conn.get_peers()
            .map(|data| {
                println!("Peers: {:?}", data);
            })
        })
    }).expect("successful request");
}
