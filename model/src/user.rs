use mongodb::bson::{Bson, Document, doc};
use rtdlib::types::UserType;
use crate::Record;

pub static COLLECTION: &str = "users";

pub static ID: &str = "id";
pub static FIRST_NAME: &str = "first_name";
pub static LAST_NAME: &str = "last_name";
pub static USERNAME: &str = "username";
pub static PHONE_NUMBER: &str = "phone_number";
pub static IS_VERIFIED: &str = "is_verified";
pub static IS_SUPPORT: &str = "is_support";
pub static RESTRICTION_REASON: &str = "restriction_reason";
pub static IS_SCAM: &str = "is_scam";
pub static USER_TYPE: &str = "user_type";
pub static SCAM: &str = "scam";
pub static BYPASS: &str = "bypass";

#[derive(Debug)]
pub struct User {
    id: i64,
    pub first_name: String,
    last_name: String,
    username: String,
    phone_number: String,
    is_verified: bool,
    is_support: bool,
    restriction_reason: String,
    is_scam: bool,
    user_type: String,
    pub scam: bool,
    bypass: bool
}

impl User {

    pub fn from_td(user: &rtdlib::types::User, scam: bool) -> Self {
        User {
            id: user.id(),
            first_name: user.first_name().to_string(),
            last_name: user.last_name().to_string(),
            username: user.username().to_string(),
            phone_number: user.phone_number().to_string(),
            is_verified: user.is_verified(),
            is_support: user.is_support(),
            restriction_reason: user.restriction_reason().to_string(),
            is_scam: user.is_scam(),
            user_type: match user.type_() {
                UserType::Bot(_) => "Bot",
                UserType::Deleted(_) => "Deleted",
                UserType::Regular(_) => "Regular",
                UserType::Unknown(_) => "Unknown",
                _ => ""
            }.to_string(),
            scam,
            bypass: false
        }
    }

    pub fn is_scam_by_admin(&self) -> bool {
        self.scam
    }

    pub fn is_bypass(&self) -> bool {
        self.bypass
    }

}

impl Record for User {

    fn from_doc(doc: &Document) -> Self {
        User {
            id: doc.get(ID).and_then(Bson::as_i64).unwrap(),
            first_name: doc.get(FIRST_NAME).and_then(Bson::as_str).unwrap().to_string(),
            last_name: doc.get(LAST_NAME).and_then(Bson::as_str).or(Some("")).unwrap().to_string(),
            username: doc.get(USERNAME).and_then(Bson::as_str).or(Some("")).unwrap().to_string(),
            phone_number: doc.get(PHONE_NUMBER).and_then(Bson::as_str).or(Some("")).unwrap().to_string(),
            is_verified: doc.get(IS_VERIFIED).and_then(Bson::as_bool).or(Some(false)).unwrap(),
            is_support: doc.get(IS_SUPPORT).and_then(Bson::as_bool).or(Some(false)).unwrap(),
            restriction_reason: doc.get(RESTRICTION_REASON).and_then(Bson::as_str).or(Some("")).unwrap().to_string(),
            is_scam: doc.get(IS_SCAM).and_then(Bson::as_bool).or(Some(false)).unwrap(),
            user_type: doc.get(USER_TYPE).and_then(Bson::as_str).or(Some("")).unwrap().to_string(),
            scam: doc.get(SCAM).and_then(Bson::as_bool).or(Some(false)).unwrap(),
            bypass: doc.get(BYPASS).and_then(Bson::as_bool).or(Some(false)).unwrap()
        }
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: self.id,
            FIRST_NAME: self.first_name.to_string(),
            LAST_NAME: self.last_name.to_string(),
            USERNAME: self.username.to_string(),
            PHONE_NUMBER: self.phone_number.to_string(),
            IS_VERIFIED: self.is_verified,
            IS_SUPPORT: self.is_support,
            RESTRICTION_REASON: self.restriction_reason.to_string(),
            IS_SCAM: self.is_scam,
            USER_TYPE: self.user_type.to_string(),
            SCAM: self.scam,
            BYPASS: self.bypass
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: self.id,
                FIRST_NAME: self.first_name.to_string(),
                LAST_NAME: self.last_name.to_string(),
                USERNAME: self.username.to_string(),
                PHONE_NUMBER: self.phone_number.to_string(),
                IS_VERIFIED: self.is_verified,
                IS_SUPPORT: self.is_support,
                RESTRICTION_REASON: self.restriction_reason.to_string(),
                IS_SCAM: self.is_scam,
                USER_TYPE: self.user_type.to_string(),
                SCAM: self.scam,
                BYPASS: self.bypass
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
        if self.phone_number.is_empty() {
            self.phone_number = new.phone_number.clone();
        }
        self.first_name = new.first_name.clone();
        self.last_name = new.last_name.clone();
        self.username = new.username.clone();
        self
    }

}
