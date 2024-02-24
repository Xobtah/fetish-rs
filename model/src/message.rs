use mongodb::bson::{Bson, Document, doc};
use rtdlib::types::{MessageContent, MessageSenderUser};
use crate::Record;

pub static COLLECTION: &str = "messages";

pub static ID: &str = "id";
pub static SENDER: &str = "sender";
pub static CHAT_ID: &str = "chat_id";
pub static DATE: &str = "date";
pub static EDIT_DATE: &str = "edit_date";
pub static RESTRICTION_REASON: &str = "restriction_reason";
pub static TYPE: &str = "type";
pub static CONTENT: &str = "content";
pub static EXTRA: &str = "extra";
pub static IS_SCAM: &str = "is_scam";
pub static TRIGGER: &str = "trigger";

pub struct Message {
    id: i64,
    sender: i64,
    chat_id: i64,
    date: i64,
    edit_date: i64,
    restriction_reason: String,
    type_: String,
    content: String,
    extra: Vec<String>,
    is_scam: bool,
    trigger: bool
}

impl Message {

    pub fn from_td(message: &rtdlib::types::Message, trigger: bool) -> Self {
        Message {
            id: message.id(),
            sender: message.sender().as_user().or(Some(&MessageSenderUser::builder().user_id(0).build())).unwrap().user_id(),
            chat_id: message.chat_id(),
            date: message.date(),
            edit_date: message.edit_date(),
            restriction_reason: message.restriction_reason().to_string(),
            type_: match message.content() {
                MessageContent::MessageAnimation(_) => "MessageAnimation",
                MessageContent::MessageAudio(_) => "MessageAudio",
                MessageContent::MessageBasicGroupChatCreate(_) => "MessageBasicGroupChatCreate",
                MessageContent::MessageCall(_) => "MessageCall",
                MessageContent::MessageChatAddMembers(_) => "MessageChatAddMembers",
                MessageContent::MessageChatChangePhoto(_) => "MessageChatChangePhoto",
                MessageContent::MessageChatChangeTitle(_) => "MessageChatChangeTitle",
                MessageContent::MessageChatDeleteMember(_) => "MessageChatDeleteMember",
                MessageContent::MessageChatDeletePhoto(_) => "MessageChatDeletePhoto",
                MessageContent::MessageChatJoinByLink(_) => "MessageChatJoinByLink",
                MessageContent::MessageChatSetTtl(_) => "MessageChatSetTtl",
                MessageContent::MessageChatUpgradeFrom(_) => "MessageChatUpgradeFrom",
                MessageContent::MessageChatUpgradeTo(_) => "MessageChatUpgradeTo",
                MessageContent::MessageContact(_) => "MessageContact",
                MessageContent::MessageContactRegistered(_) => "MessageContactRegistered",
                MessageContent::MessageCustomServiceAction(_) => "MessageCustomServiceAction",
                MessageContent::MessageDice(_) => "MessageDice",
                MessageContent::MessageDocument(_) => "MessageDocument",
                MessageContent::MessageExpiredPhoto(_) => "MessageExpiredPhoto",
                MessageContent::MessageExpiredVideo(_) => "MessageExpiredVideo",
                MessageContent::MessageGame(_) => "MessageGame",
                MessageContent::MessageGameScore(_) => "MessageGameScore",
                MessageContent::MessageInvoice(_) => "MessageInvoice",
                MessageContent::MessageLocation(_) => "MessageLocation",
                MessageContent::MessagePassportDataReceived(_) => "MessagePassportDataReceived",
                MessageContent::MessagePassportDataSent(_) => "MessagePassportDataSent",
                MessageContent::MessagePaymentSuccessful(_) => "MessagePaymentSuccessful",
                MessageContent::MessagePaymentSuccessfulBot(_) => "MessagePaymentSuccessfulBot",
                MessageContent::MessagePhoto(_) => "MessagePhoto",
                MessageContent::MessagePinMessage(_) => "MessagePinMessage",
                MessageContent::MessagePoll(_) => "MessagePoll",
                MessageContent::MessageProximityAlertTriggered(_) => "MessageProximityAlertTriggered",
                MessageContent::MessageScreenshotTaken(_) => "MessageScreenshotTaken",
                MessageContent::MessageSticker(_) => "MessageSticker",
                MessageContent::MessageSupergroupChatCreate(_) => "MessageSupergroupChatCreate",
                MessageContent::MessageText(_) => "MessageText",
                MessageContent::MessageUnsupported(_) => "MessageUnsupported",
                MessageContent::MessageVenue(_) => "MessageVenue",
                MessageContent::MessageVideo(_) => "MessageVideo",
                MessageContent::MessageVideoNote(_) => "MessageVideoNote",
                MessageContent::MessageVoiceNote(_) => "MessageVoiceNote",
                MessageContent::MessageWebsiteConnected(_) => "MessageWebsiteConnected",
                _ => "None"
            }.to_string(),
            content: match message.content() {
                MessageContent::MessagePhoto(message_photo) => message_photo.caption().text(),
                MessageContent::MessageVideo(message_video) => message_video.caption().text(),
                MessageContent::MessageText(message_text) => message_text.text().text(),
                _ => ""
            }.to_string(),
            extra: vec![],
            is_scam: false,
            trigger
        }
    }

}

impl Record for Message {

    fn from_doc(doc: &Document) -> Self {
        Message {
            id: doc.get(ID).and_then(Bson::as_i64).unwrap(),
            sender: doc.get(SENDER).and_then(Bson::as_i64).unwrap(),
            chat_id: doc.get(CHAT_ID).and_then(Bson::as_i64).unwrap(),
            date: doc.get(DATE).and_then(Bson::as_i64).unwrap(),
            edit_date: doc.get(EDIT_DATE).and_then(Bson::as_i64).unwrap(),
            restriction_reason: doc.get(RESTRICTION_REASON).and_then(Bson::as_str).unwrap().to_string(),
            type_: doc.get(TYPE).and_then(Bson::as_str).unwrap().to_string(),
            content: doc.get(CONTENT).and_then(Bson::as_str).unwrap().to_string(),
            extra: doc.get(EXTRA).and_then(Bson::as_array).unwrap().iter().map(|bson| bson.as_str().unwrap().to_string()).collect(),
            is_scam: doc.get(IS_SCAM).and_then(Bson::as_bool).unwrap(),
            trigger: doc.get(TRIGGER).and_then(Bson::as_bool).unwrap()
        }
    }

    fn to_doc(&self) -> Document {
        return doc! {
            ID: self.id,
            SENDER: self.sender,
            CHAT_ID: self.chat_id,
            DATE: self.date,
            EDIT_DATE: self.edit_date,
            RESTRICTION_REASON: self.restriction_reason.to_string(),
            TYPE: self.type_.to_string(),
            CONTENT: self.content.to_string(),
            EXTRA: self.extra.clone(),
            IS_SCAM: self.is_scam,
            TRIGGER: self.trigger
        };
    }

    fn to_doc_update(&self) -> Document {
        return doc! {
            "$set": {
                ID: self.id,
                SENDER: self.sender,
                CHAT_ID: self.chat_id,
                DATE: self.date,
                EDIT_DATE: self.edit_date,
                RESTRICTION_REASON: self.restriction_reason.to_string(),
                TYPE: self.type_.to_string(),
                CONTENT: self.content.to_string(),
                EXTRA: self.extra.clone(),
                IS_SCAM: self.is_scam,
                TRIGGER: self.trigger
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
        self.content = new.content.clone();
        self
    }

}
