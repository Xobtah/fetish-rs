use log::info;
use unidecode::unidecode;
use rtdlib::types::{Message, MessageContent::*, MessageSender, User};
use config::Config;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::mongo::Mongo;
use model::{Keywords, KeywordMatch, ScamType, Record, ForbiddenNames};
use crate::recorder::Recorder;

#[derive(Clone)]
pub struct ScamAnalyser<T: Recorder> {
    config: Config,
    mongo: T
}

impl ScamAnalyser<Mongo> {

    pub fn new(config: Config, mongo: Mongo) -> Self {
        ScamAnalyser {
            config,
            mongo
        }
    }

    pub fn is_threat(&self, message: &Message) -> bool {
        if message.is_outgoing() {
            info!("[{}] is an output message", message.id());
            return false;
        }

        if let MessageSender::User(message_sender_user) = message.sender() {
            if let Some(doc) = self.mongo.get_doc(model::USERS_COLLECTION, message_sender_user.user_id()).unwrap() {
                let user = model::User::from_doc(&doc);
                if user.is_bypass() {
                    info!("[{}] has the n-word pass", user.first_name);
                    return false;
                }
            }
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if now as i64 - message.date() >= self.config.sender.timeout as i64 {
            info!("[{}] is older than {} seconds", message.id(), self.config.sender.timeout);
            return false;
        }

        true
    }

    pub fn analyse(&self, message: &Message) -> Vec<ScamType> {
        let content = message.content();
        let mut ret = Vec::new();

        let text = match content {
            MessagePhoto(message_photo) => Some(message_photo.caption().text()),
            MessageVideo(message_video) => Some(message_video.caption().text()),
            MessageText(message_text) => Some(message_text.text().text()),
            _ => None
        };

        // Keyword scam
        if let Some(text) = text {
            let text = text.to_uppercase();
            if self.is_keyword_in_text(&text) || text.eq("CC") {
                ret.push(ScamType::Keyword);
            }
        }

        // Account scam
        if let MessageSender::User(message_sender) = message.sender() {
            if self.is_scammer_account(message_sender.user_id()) {
                ret.push(ScamType::Account(message_sender.user_id()));
            }
        }

        ret
    }

    fn is_keyword_in_text(&self, text: &str) -> bool {
        let text = unidecode(text);

        if let KeywordMatch::NoneMatch = Keywords::from_doc(&self.mongo.get_doc(model::CONFIG_COLLECTION, model::KEYWORDS_ID).unwrap().unwrap()).text_match(&text) {
            false
        } else {
            true
        }
    }

    fn is_scammer_account(&self, user_id: i64) -> bool {
        if let Some(doc) = self.mongo.get_doc(model::USERS_COLLECTION, user_id).unwrap() {
            model::User::from_doc(&doc).is_scam_by_admin()
        } else {
            false
        }
    }

    pub fn is_new_user_scam(&self, user: &User) -> bool {
        let first_name = unidecode(user.first_name());
        let last_name = unidecode(user.last_name());
        let username = unidecode(user.username());

        // Username contains "ESCORT"
        if username.to_uppercase().contains("ESCORT") {
            return true;
        }

        // First name equals last name
        if first_name.to_uppercase().eq(&last_name.to_uppercase()) {
            return true;
        }

        // TODO : Same name in DB

        // Forbidden names
        if ForbiddenNames::from_doc(&self.mongo.get_doc(model::CONFIG_COLLECTION, model::FORBIDDEN_NAMES_ID).unwrap().unwrap())
            .name_match(&first_name, &last_name) {
            return true;
        }

        false
    }

}

/*
 curl -F 'image=@/c/Users/sylva/Desktop/go.jpg' -H 'api-key:e84953bb-bc77-4f9b-9245-d2aa8cc6268c' https://api.deepai.org/api/nsfw-detector
{
    "id": "1843aafc-292d-4c74-bc48-f8e3e88f7c78",
    "output": {
        "detections": [
            {
                "confidence": "0.73",
                "bounding_box": [
                    462,
                    437,
                    256,
                    206
                ],
                "name": "Female Breast - Covered"
            }
        ],
        "nsfw_score": 0.27433815598487854
    }
}
 */

/*#[cfg(test)]
mod tests {
    use crate::ScamAnalyser;
    use crate::recorder::Recorder;
    use mongodb::bson::Document;
    use mongodb::error::Error;
    use model::Record;

    struct MongoMock;

    impl Recorder for MongoMock {

        fn get_doc(&self, collection_name: &str, id: i64) -> Result<Option<Document>, Error> {
            todo!()
        }

        fn save_doc(&self, doc: impl Record) -> Result<(), Error> {
            todo!()
        }

    }

    #[test]
    fn test1() {
        let message = "Coucou je suis disponible pour des rencontres coquines intéressé veuillez me contacter 💯💯💞💯💞";

        assert!(is_keyword_in_text(message));
    }

    #[test]
    fn test2() {
        let message = "Salut ici je suis disponible pour des plans cul et sexcam si tu es intéressé écrit moi en privé";

        assert!(is_keyword_in_text(message));
    }

    #[test]
    fn test3() {
        let message = "Iks creent tous des snap mtn";

        assert!(!is_keyword_in_text(message));
    }

}*/
