use simple_logger::SimpleLogger;

use crate::fetish::Fetish;

mod thelp;
mod tgfn;
mod auth;
mod message_sender;
mod fetish;
mod mongo;
mod recorder;
mod scam_analyser;

fn main() {
    SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Debug);
    let args: Vec<String> = std::env::args().collect();

    Fetish::new(args.get(1).unwrap()).run();
}
