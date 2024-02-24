mod user;
mod message;
mod chat;
mod config;
mod stats;
mod record;
mod sanction;

pub use user::User;
pub use message::Message;
pub use chat::Chat;
pub use config::{Keywords, KeywordMatch, ForbiddenNames};
pub use stats::MessageSent;
pub use record::Record;
pub use sanction::{ScamType, Sanction};

pub use user::COLLECTION as USERS_COLLECTION;
pub use message::COLLECTION as MESSAGES_COLLECTION;
pub use chat::COLLECTION as CHATS_COLLECTION;
pub use config::COLLECTION as CONFIG_COLLECTION;
pub use sanction::COLLECTION as STATS_COLLECTION;

pub use config::KEYWORDS_ID;
pub use config::FORBIDDEN_NAMES_ID;
