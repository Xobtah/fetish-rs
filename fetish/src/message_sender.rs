use log::{info, debug};
use rtdlib::types::{SendMessage, InputMessageContent, InputMessageText, FormattedText};
use rand::Rng;
use config::Config;
use telegram_client::api::aevent::EventApi;
use std::sync::mpsc::Receiver;
use crate::mongo::Mongo;
use model::{Sanction, ScamType};
use crate::recorder::Recorder;

pub struct Sender {
    config: Config,
    api: EventApi,
    mongo: Mongo,
    rx: Receiver<Sanction>
}

impl Sender {

    pub fn new(config: Config, api: EventApi, mongo: Mongo, rx: Receiver<Sanction>) -> Self {
        Sender {
            config: config.clone(),
            api,
            mongo,
            rx
        }
    }

    pub fn run(self) {
        for sanction in &self.rx {
            self.send(&sanction);
        }
    }

    fn load_text(&self, scam_types: &Vec<ScamType>) -> String {
        let mut txt = String::new();

        for scam_type in scam_types {
            match scam_type {
                ScamType::Keyword => {
                    if txt.is_empty() {
                        txt.push_str(std::fs::read_to_string(self.config.message_path.as_str()).unwrap().as_str());
                    }
                },
                ScamType::Account(_) => {
                    //txt = std::fs::read_to_string(self.config.scammer_account_path.as_str()).unwrap(); // FIXME
                    txt = String::from("Ceci est le compte d'un arnaqueur. S'il vous demande quoi que ce soit, bloquez-le, autrement il tentera de voler votre argent.");
                }
            };
        }

        // About section
        txt.push_str("\n\n----------\n\n");
        //txt.push_str(std::fs::read_to_string(self.config.about_path.as_str()).unwrap().as_str());
        txt.push_str("Pour plus d'informations sur les arnaques, rejoignez le canal ScamWatch : https://t.me/thescamwatch"); // FIXME

        txt
    }

    fn send(&self, sanction: &Sanction) {
        info!("Sending sanction for message id [{}]", sanction.message().id());

        // Load artillery
        let txt = self.load_text(sanction.scam_types());

        // Wait to be more human
        let (min, max) = (self.config.sender.min_wait, self.config.sender.max_wait);
        let waiting_time = rand::thread_rng().gen::<f64>() * (max - min) + min;
        info!("Waiting {} seconds", waiting_time);
        std::thread::sleep(std::time::Duration::from_secs(waiting_time as u64));

        // Fire
        if self.config.sender.send {
            self.api.send_message(SendMessage::builder()
                .chat_id(sanction.message().chat_id())
                .input_message_content(InputMessageContent::input_message_text(InputMessageText::builder()
                    .text(FormattedText::builder().text(&txt))
                    .clear_draft(true)
                    .disable_web_page_preview(true)
                    .build()))
                .reply_to_message_id(sanction.message().id())
                .build()).unwrap();
            info!("PROD : Repression sent");
        } else {
            info!("DEV : Repression not sent");
            debug!("The message would have been :\n{}", txt);
        }

        self.mongo.save_doc(sanction).unwrap()
    }

}
