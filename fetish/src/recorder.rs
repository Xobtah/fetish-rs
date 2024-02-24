use mongodb::bson::Document;
use mongodb::error::Error;
use model::Record;

pub trait Recorder {

    fn get_doc(&self, collection_name: &str, id: i64) -> Result<Option<Document>, Error>;
    fn save_doc(&self, doc: impl Record) -> Result<(), Error>;

}