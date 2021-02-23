use std::collections::HashMap;
use leaky_bucket::LeakyBucket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Data};
use std::time::Duration;
use rocket::http::RawStr;
use std::borrow::Borrow;
use rocket::tokio::sync::Mutex;

pub struct RateLimiter {
    pub rl: Mutex<HashMap<String, LeakyBucket>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            rl: Mutex::new(HashMap::new())
        }
    }
}

#[rocket::async_trait]
impl Fairing for RateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "Rate Limiter",
            kind: Kind::Request
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data) {
        let id: String = req.get_query_value("id")
            .and_then(|r| r.ok())
            .unwrap_or("not_found".parse().unwrap());

        // block until lock can be held
        let clone = &self.rl;
        let mut users = clone.lock().await;

        let user = users.entry(id).or_insert({
            LeakyBucket::builder()
                .max(3)
                .tokens(3)
                .refill_interval(Duration::new(5, 0))
                .build()
                .unwrap()
        });

        user.acquire_one().await;
    }
}