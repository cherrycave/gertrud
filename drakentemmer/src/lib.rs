use async_lock::Semaphore;

mod category;
pub use category::*;

pub struct Drakentemmer {
    base_url: String,
    client: reqwest::Client,
    semaphore: Semaphore,
}

impl Drakentemmer {
    pub fn new(
        base_url: String,
        client: Option<reqwest::Client>,
        concurrent_requests: Option<usize>,
    ) -> Drakentemmer {
        Drakentemmer {
            base_url,
            client: client.unwrap_or_else(reqwest::Client::new),
            semaphore: Semaphore::new(concurrent_requests.unwrap_or(1)),
        }
    }
}
