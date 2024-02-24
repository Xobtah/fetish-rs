use mongodb::bson::{Bson, Document, doc};
use crate::Record;

pub static COLLECTION: &str = "config";
pub static KEYWORDS_ID: i64 = 0;
pub static FORBIDDEN_NAMES_ID: i64 = 1;

pub static ID: &str = "id";
pub static FR: &str = "fr";
pub static EN: &str = "en";
pub static DE: &str = "de";
pub static NAMES: &str = "names";

pub struct Keywords {
    fr: Vec<String>,
    en: Vec<String>,
    de: Vec<String>
}
pub struct ForbiddenNames {
    names: Vec<String>
}

pub enum KeywordMatch {
    FrMatch,
    EnMatch,
    DeMatch,
    NoneMatch
}

impl Keywords {

    pub fn text_match(&self, text: &str) -> KeywordMatch {
        let ok = |keywords: &Vec<String>| keywords.iter().any(|key| text.to_uppercase().contains(key));

        if ok(&self.fr) {
            KeywordMatch::FrMatch
        } else if ok(&self.en) {
            KeywordMatch::EnMatch
        } else if ok(&self.de) {
            KeywordMatch::DeMatch
        } else {
            KeywordMatch::NoneMatch
        }
    }

}

impl Record for Keywords {

    fn from_doc(doc: &Document) -> Self {
        Keywords {
            fr: doc.get(FR).and_then(Bson::as_array).unwrap().iter().map(|bson| bson.as_str().unwrap().to_string().to_uppercase()).collect(),
            en: doc.get(EN).and_then(Bson::as_array).unwrap().iter().map(|bson| bson.as_str().unwrap().to_string().to_uppercase()).collect(),
            de: doc.get(DE).and_then(Bson::as_array).unwrap().iter().map(|bson| bson.as_str().unwrap().to_string().to_uppercase()).collect()
        }
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: KEYWORDS_ID,
            FR: self.fr.clone(),
            EN: self.en.clone(),
            DE: self.de.clone()
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: KEYWORDS_ID,
                FR: self.fr.clone(),
                EN: self.en.clone(),
                DE: self.de.clone()
            }
        };
    }

    fn collection_name(&self) -> &str {
        COLLECTION
    }

    fn id(&self) -> i64 {
        KEYWORDS_ID
    }

    fn merge(&mut self, _new: &Self) -> &mut Self {
        todo!()
    }

}

impl ForbiddenNames {

    pub fn name_match(&self, first_name: &str, last_name: &str) -> bool {
        self.names.iter().any(|name| first_name.to_uppercase().contains(name) || last_name.to_uppercase().contains(name))
    }

}

impl Record for ForbiddenNames {

    fn from_doc(doc: &Document) -> Self {
        ForbiddenNames {
            names: doc.get(NAMES).and_then(Bson::as_array).unwrap().iter().map(|bson| bson.as_str().unwrap().to_string().to_uppercase()).collect()
        }
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: FORBIDDEN_NAMES_ID,
            NAMES: self.names.clone()
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: FORBIDDEN_NAMES_ID,
                NAMES: self.names.clone()
            }
        };
    }

    fn collection_name(&self) -> &str {
        COLLECTION
    }

    fn id(&self) -> i64 {
        FORBIDDEN_NAMES_ID
    }

    fn merge(&mut self, _new: &Self) -> &mut Self {
        todo!()
    }

}
