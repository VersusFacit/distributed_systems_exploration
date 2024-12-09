use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Mutex;

use warp::Filter;

use distributed_systems_exploration as lib;
use lib::visitor_tracking::VisitorLog;

#[tokio::main]
async fn main() {
    let server_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(server_addr, 8085);

    let visitor_log = Arc::new(Mutex::new(VisitorLog::new()));

    tracing_subscriber::fmt()
        //.with_max_level(tracing::Level::DEBUG)
        .init();

    //std::env::set_var("RUST_LOG", "trace");
    let route = warp::path!("hey")
        .and(warp::any().map(move || visitor_log.clone()))
        .and_then(lib::visitor_tracking::log_visitor);
    warp::serve(route).run(socket).await;
}
