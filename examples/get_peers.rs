use std::env;

extern crate tk_easyloop;
extern crate tk_cantal;
extern crate env_logger;

use tk_cantal::connect_local;

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "warn");
    }
    env_logger::init();
    let data = tk_easyloop::run(|| {
        let conn = connect_local(&tk_easyloop::handle());
        conn.get_peers()
    }).expect("successful request");
    println!("Peers: {:?}", data);
}
