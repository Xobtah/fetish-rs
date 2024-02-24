use mongodb::bson::Document;

pub trait Record {

    fn from_doc(doc: &Document) -> Self;
    fn to_doc(&self) -> Document;
    fn to_doc_update(&self) -> Document;

    fn collection_name(&self) -> &str;
    fn id(&self) -> i64;

    fn merge(&mut self, new: &Self) -> &mut Self;

}
