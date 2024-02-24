use log::info;
use mongodb::sync::Client;
use config::Config;
use mongodb::bson::{doc, Document};
use mongodb::error::Error;
use model::Record;
use crate::recorder::Recorder;

#[derive(Clone)]
pub struct Mongo {
    config: Config,
    mdb: Client,
    db: String
}

impl Mongo {

    pub fn new(config: Config) -> Self {
        Mongo {
            config: config.clone(),
            mdb: mongodb::sync::Client::with_uri_str(config.mongo.url.as_str()).unwrap(),
            db: String::from("fetish")
        }
    }

}

impl Recorder for Mongo {

    fn get_doc(&self, collection_name: &str, id: i64) -> Result<Option<Document>, Error> {
        let collection = self.mdb.database(self.db.as_str()).collection(collection_name);
        collection.find_one(doc! { "id": id }, None)
    }

    fn save_doc(&self, model: impl Record) -> Result<(), Error> {
        let collection = self.mdb.database(self.db.as_str()).collection(model.collection_name());

        if self.get_doc(model.collection_name(), model.id())?.is_none() {
            info!("Saving [{}] in '{}'", model.id(), model.collection_name());
            collection.insert_one(model.to_doc(), None)?;
        } else {
            info!("Updating [{}] in '{}'", model.id(), model.collection_name());
            collection.update_one(doc! { "id": model.id() }, model.to_doc_update(), None)?;
        }

        Ok(())
    }

}
