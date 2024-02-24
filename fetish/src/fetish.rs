use log::{info, debug, error};
use config::{Config, LogType};
use telegram_client::api::Api;
use telegram_client::client::Client;
use crate::auth;
use rtdlib::types::{RObject, GetUser, MessageSender};
use std::sync::{Arc, Mutex, mpsc};
use telegram_client::api::aevent::EventApi;
use crate::scam_analyser::{ScamAnalyser};
use telegram_client::listener::Listener;
use rtdlib::types::MessageContent::{MessagePhoto, MessageVideo, MessageText};
use crate::mongo::Mongo;
use std::sync::mpsc::Sender;
use model::{Sanction, Record};
use crate::recorder::Recorder;

#[derive(Clone)]
pub struct Fetish {
    config: Config,
    api: EventApi,
    analyser: ScamAnalyser<Mongo>,
    tx: Arc<Mutex<Sender<Sanction>>>,
    mongo: Mongo
}

impl Fetish {

    pub fn new(conf_path: &str) -> Self {
        let config = get_config(conf_path);
        let api = Api::event();
        let mongo = Mongo::new(config.clone());

        let (tx, rx) = mpsc::channel();

        let sender_config = config.clone();
        let sender_api = api.clone();
        let sender_mongo = mongo.clone();
        std::thread::spawn(move || {
            crate::message_sender::Sender::new(sender_config, sender_api, sender_mongo, rx).run();
        });

        Fetish {
            config: config.clone(),
            api: api.clone(),
            analyser: ScamAnalyser::new(config.clone(), mongo.clone()),
            tx: Arc::new(Mutex::new(tx)),
            mongo
        }
    }

    pub fn run(&mut self) {
        let mut client = Client::new(self.api.api().clone());
        client.warn_unregister_listener(false);
        let listener = client.listener();

        auth::auth(listener, self.config.session_path.clone());

        self.listen_new_messages(listener);
        self.listen_new_chats(listener);
        self.listen_new_users(listener);

        client.daemon("fetish-rs").expect("Failed to start daemon");
    }

    fn listen_new_messages(&self, listener: &mut Listener) {
        let fetish = self.clone();

        listener.on_update_new_message(move |(api, update)| {
            let message = update.message().clone();

            // Save user in DB if doesn't exist
            if let MessageSender::User(message_sender_user) = message.sender() {
                if fetish.mongo.get_doc(model::USERS_COLLECTION, message_sender_user.user_id()).unwrap().is_none() {
                    api.get_user(GetUser::builder().user_id(message_sender_user.user_id()).build()).unwrap();
                }
            }

            info!("Getting new message");
            debug!("Message, from: '{:?}', data: {}", message.sender(), message.to_json().expect("Can't serialize json"));

            ///// SHOW MESSAGE IN CONSOLE
            if let Some(content) = match message.content() {
                MessagePhoto(message_photo) => Some(message_photo.caption().text()),
                MessageVideo(message_video) => Some(message_video.caption().text()),
                MessageText(message_text) => Some(message_text.text().text()),
                _ => None
            } {
                debug!("MESSAGE'S CONTENT : {}", content);
            }
            /////

            // Analyse threat
            if !fetish.analyser.is_threat(&message) {
                info!("The message is not a threat");
                return Ok(());
            }
            info!("The message is a threat");

            // Analyse scam
            let analyse = fetish.analyser.analyse(&message);

            if analyse.is_empty() {
                info!("The message is not a scam");
                // Save message in DB
                fetish.mongo.save_doc(model::Message::from_td(&message, false)).unwrap();
                return Ok(());
            } else {
                info!("SCAM DETECTED !!!");
                info!("SCAM DETECTED !!!");
                info!("SCAM DETECTED !!!");
                // Save message in DB
                fetish.mongo.save_doc(model::Message::from_td(&message, true)).unwrap();
            }

            if message.chat_id() < 0 {
                if let Err(e) = fetish.tx.lock().unwrap().send(Sanction::new(message.clone(), analyse)) {
                    error!("FAILED TO SEND SANCTION : {}", e.to_string());
                }
            } else {
                info!("This is a private chat, not sending the message");
            }

            Ok(())
        });
    }

    fn listen_new_chats(&self, listener: &mut Listener) {
        let fetish = self.clone();

        listener.on_update_new_chat(move |(_api, update)| {
            let chat = update.chat();
            info!("Chat {} info", chat.title());

            // Save chat in DB
            match fetish.mongo.get_doc(model::CHATS_COLLECTION, chat.id()) {
                Ok(Some(_)) => info!("Chat '{}' already exists in DB", chat.id()),
                Ok(None) => if let Err(e) = fetish.mongo.save_doc(model::Chat::from_td(&chat)) { error!("Failed to save chat '{}' in DB : {:?}", chat.id(), e); },
                Err(e) => error!("Failed to get chat '{}' from DB : {:?}", chat.id(), e)
            }

            Ok(())
        });
    }

    fn listen_new_users(&self, listener: &mut Listener) {
        let fetish = self.clone();

        listener.on_update_user(move |(_api, update)| {
            let user = update.user();
            info!("User '{} {}' info", user.first_name(), user.last_name());

            // Save user in DB
            match fetish.mongo.get_doc(model::USERS_COLLECTION, user.id()) {
                Ok(Some(u)) => {
                    info!("User '{}' already exists in DB, update", user.id());
                    let mut user_updt = model::User::from_doc(&u);
                    if !user_updt.scam && !user_updt.is_bypass() {
                        info!("Checking if {} {} is a scammer", user.first_name(), user.last_name());
                        user_updt.scam = fetish.analyser.is_new_user_scam(&user);
                        if user_updt.scam {
                            info!("Updated user {} {} is a scammer", user.first_name(), user.last_name());
                        } else {
                            info!("Updated user {} {} may not be a scammer", user.first_name(), user.last_name());
                        }
                    }
                    user_updt.merge(&model::User::from_td(user, user_updt.scam));
                    debug!("New user info {:?}", user_updt);
                    if let Err(e) = fetish.mongo.save_doc(user_updt) {
                        error!("Failed to update user '{}' in DB : {:?}", user.id(), e);
                    }
                },
                Ok(None) => {
                    let scam = fetish.analyser.is_new_user_scam(&user);
                    if scam {
                        info!("New user {} {} is a scammer", user.first_name(), user.last_name());
                    }
                    if let Err(e) = fetish.mongo.save_doc(model::User::from_td(&user, scam)) {
                        error!("Failed to save user '{}' in DB : {:?}", user.id(), e);
                    }
                },
                Err(e) => error!("Failed to get user '{}' from DB : {:?}", user.id(), e)
            }

            Ok(())
        });
    }

}

fn get_config(conf_path: &str) -> Config {
    let config = Config::from(conf_path);

    config.log().map(|v| {
        Client::set_log_verbosity_level(v.level.clone() as i32).unwrap();

        if v.type_ == LogType::File {
            v.path.clone().map(|v| {
                Client::set_log_file_path(Some(&v[..]));
            });
        }
    });

    config
}
