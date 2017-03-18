use std::env;

extern crate tk_easyloop;
extern crate env_logger;

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "warn");
    }
    env_logger::init().unwrap();
    let data = tk_easyloop::run(|| {
        let conn = connect_local();
        conn.get_peers()
    }).expect("successful request");
    println!("Peers: {:?}", data);
}
