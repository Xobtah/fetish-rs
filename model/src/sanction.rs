use mongodb::bson::{Document, doc};
use rtdlib::types::Message;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::Record;

pub static COLLECTION: &str = "stats";

pub static ID: &str = "id";
pub static MESSAGE: &str = "message";
pub static SCAM_TYPES: &str = "scam-types";
pub static DATE: &str = "date";

pub struct Sanction {
    message: Message,
    scam_types: Vec<crate::sanction::ScamType>,
    date: Option<u64>
}

pub enum ScamType {
    Keyword,
    Account(i64)
}

impl Sanction {

    pub fn new(message: Message, scam_types: Vec<crate::sanction::ScamType>) -> Self {
        Sanction {
            message,
            scam_types,
            date: None
        }
    }

    pub fn message(&self) -> &Message {
        &self.message
    }

    pub fn scam_types(&self) -> &Vec<ScamType> {
        &self.scam_types
    }

    pub fn date(&self) -> &Option<u64> {
        &self.date
    }

    fn scam_types_to_str(&self) -> Vec<String> {
        self.scam_types.iter()
            .map(|st| match st {
                ScamType::Keyword => String::from("Keyword"),
                ScamType::Account(_) => String::from("Account")
            })
            .collect()
    }

}

impl Record for Sanction {

    // This method should never be called
    fn from_doc(_doc: &Document) -> Self {
        todo!()
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: self.message.id(),
            MESSAGE: self.message.id(),
            SCAM_TYPES: self.scam_types_to_str(),
            DATE: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: self.message.id(),
                MESSAGE: self.message.id(),
                SCAM_TYPES: self.scam_types_to_str(),
                DATE: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            }
        };
    }

    fn collection_name(&self) -> &str {
        COLLECTION
    }

    fn id(&self) -> i64 {
        self.message.id()
    }

    fn merge(&mut self, _new: &Self) -> &mut Self {
        todo!()
    }

}

impl Record for &Sanction {

    // This method should never be called
    fn from_doc(_doc: &Document) -> Self {
        todo!()
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: self.message.id(),
            MESSAGE: self.message.id(),
            SCAM_TYPES: self.scam_types_to_str(),
            DATE: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: self.message.id(),
                MESSAGE: self.message.id(),
                SCAM_TYPES: self.scam_types_to_str(),
                DATE: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            }
        };
    }

    fn collection_name(&self) -> &str {
        COLLECTION
    }

    fn id(&self) -> i64 {
        self.message.id()
    }

    fn merge(&mut self, _new: &Self) -> &mut Self {
        todo!()
    }

}
