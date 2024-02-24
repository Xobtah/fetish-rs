use std::time::{SystemTime, UNIX_EPOCH};
use mongodb::bson::{Document, doc};

pub static TIMESTAMP: &str = "timestamp";
pub static IS_SENT: &str = "is_sent";

pub struct MessageSent {
    timestamp: u64,
    is_sent: bool
}

impl MessageSent {

    pub fn new(is_sent: bool) -> Self {
        MessageSent {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            is_sent
        }
    }

    pub fn to_doc(&self) -> Document {
        return doc! {
            TIMESTAMP: self.timestamp,
            IS_SENT: self.is_sent
        };
    }

}
