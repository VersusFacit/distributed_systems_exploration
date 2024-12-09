
pub mod visitor_tracking {
    use std::ops::Index;
    use std::sync::Arc;
    use std::time::Duration;
    use std::time::SystemTime;

    use chrono::DateTime;
    use chrono::offset::Utc;

    use tokio::sync::Mutex;
    use tracing::{debug, info, instrument};

    #[derive(Debug)]
    pub struct VisitorLog {
        data: Vec<DateTime<Utc>>
    }
    impl VisitorLog {
        pub fn new() -> Self {
            VisitorLog {
                data: vec![]
            }
        }

        pub fn add_visit(&mut self) {
            self.data.push(SystemTime::now().into())
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }
    impl Index<usize> for VisitorLog {
        type Output = DateTime<Utc>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    pub type SharedVisitorLog = Arc<Mutex<VisitorLog>>;

    #[instrument]
    pub async fn log_visitor(
        visitor_log: SharedVisitorLog
    ) -> Result<impl warp::Reply, warp::Rejection> {
        debug!(message = "[waiting for lock]");

        let mut data = visitor_log.lock().await;
        debug!(message = "[lock acquired]");

        // here for debugging the server is operating asynchronously
        if data.len() % 2 == 0 {
            tokio::time::sleep(Duration::from_secs(4)).await;
        }

        data.add_visit();
        info!(message = format!("[visitor_log after insertion] {:?}", &data));

        Ok::<_, warp::Rejection>(warp::reply::json(&data.len()))
    }

    #[cfg(test)]
    mod test {

        use super::*;

        use warp::Reply;

        #[tokio::test]
        async fn should_add_a_visitor() {
            let mut shared_visitor_log = VisitorLog::new();
            shared_visitor_log.add_visit();
            assert_eq!(shared_visitor_log.len(), 1)
        }

        #[test]
        fn should_not_overwrite_past_visit_logs() {
            let mut shared_visitor_log = VisitorLog::new();
            shared_visitor_log.add_visit();
            shared_visitor_log.add_visit();
            assert_eq!(shared_visitor_log.len(), 2);
            assert!(shared_visitor_log[0] < shared_visitor_log[1])
        }
    }
}

pub mod users {
    use std::collections::HashSet;

    pub type Group = usize;

    pub struct User {
        username: String,
        password: String, // TODO: NO
        groups: HashSet<Group>,
    }
}
