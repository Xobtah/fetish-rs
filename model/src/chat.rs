use mongodb::bson::{Bson, Document, doc};
use rtdlib::types::ChatType;
use crate::Record;

pub static COLLECTION: &str = "chats";

pub static ID: &str = "id";
pub static TITLE: &str = "title";
pub static TYPE: &str = "type";

pub struct Chat {
    id: i64,
    title: String,
    type_: String
}

impl Chat {

    pub fn from_td(chat: &rtdlib::types::Chat) -> Self {
        Chat {
            id: chat.id(),
            title: chat.title().to_string(),
            type_: match chat.type_() {
                ChatType::BasicGroup(_) => "BasicGroup",
                ChatType::Private(_) => "Private",
                ChatType::Secret(_) => "Secret",
                ChatType::Supergroup(_) => "Supergroup",
                _ => "Unknown"
            }.to_string()
        }
    }

}

impl Record for Chat {

    fn from_doc(doc: &Document) -> Self {
        Chat {
            id: doc.get(ID).and_then(Bson::as_i64).unwrap(),
            title: doc.get(TITLE).and_then(Bson::as_str).unwrap().to_string(),
            type_: doc.get(TYPE).and_then(Bson::as_str).unwrap().to_string()
        }
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: self.id,
            TITLE: self.title.to_string(),
            TYPE: self.type_.to_string()
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: self.id,
                TITLE: self.title.to_string(),
                TYPE: self.type_.to_string()
            }
        };
    }

    fn collection_name(&self) -> &str {
        COLLECTION
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn merge(&mut self, new: &Self) -> &mut Self {
        self.title = new.title.clone();
        self
    }

}
