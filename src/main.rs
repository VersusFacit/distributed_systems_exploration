use chrono::offset::Utc;
use chrono::DateTime;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tracing::{debug, info, instrument};

use warp::Filter;

type Database = Arc<Mutex<HashMap<usize, DateTime<Utc>>>>;

#[instrument]
async fn add_entry(
    shared_counter: Arc<AtomicUsize>,
    database: Database,
) -> Result<impl warp::Reply, warp::Rejection> {
    let counter = shared_counter.fetch_add(1, Ordering::Relaxed);
    {
        debug!(
            message = "[counter after uptick]",
            counter = %counter,
        );
        debug!(
            message = "[waiting for lock]",
            counter = %counter,
        );

        // here for debugging the server
        if counter % 2 == 0 {
            tokio::time::sleep(std::time::Duration::from_secs(4)).await;
        }

        let mut data = database.lock().await;
        debug!(
            message = "[lock acquired]",
            counter = %counter,
        );

        data.insert(counter, SystemTime::now().into());
        info!(
            message = format!("[database after insertion] {:?}", &data),
            counter = %counter,
        );
    }

    Ok::<_, warp::Rejection>(warp::reply::json(&format!("success {counter}")))
}

#[tokio::main]
async fn main() {
    let server_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let socket = SocketAddr::new(server_addr, 8085);

    let counter = Arc::new(AtomicUsize::new(0));
    let database = Arc::new(Mutex::new(HashMap::<usize, DateTime<Utc>>::new()));

    tracing_subscriber::fmt()
        //.with_max_level(tracing::Level::DEBUG)
        .init();

    //std::env::set_var("RUST_LOG", "trace");
    let route = warp::path!("hey")
        .and(warp::any().map(move || counter.clone()))
        .and(warp::any().map(move || database.clone()))
        .and_then(add_entry);
    warp::serve(route).run(socket).await;
}
