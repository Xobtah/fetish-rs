use std::path::Path;
use log::debug;
use urlencoding;
use rtdlib::types::*;

#[derive(Debug, Clone)]
pub struct Config {
    toml: toml::Value,
    pub sender: Sender,
    pub mongo: Mongo,
    pub keywords_path: String,
    pub message_path: String,
    pub scammer_account_path: String,
    pub about_path: String,
    pub session_path: String
}

impl Default for Config {
    fn default() -> Self {
        let toml_file = match hostname::get_hostname() {
            Some(name) => format!("telegram-client.{}.toml", name),
            None => "telegram-client.toml".to_string()
        };
        let mut toml_file = Path::new("config/res").join(&toml_file[..]);
        if !toml_file.exists() {
            toml_file = toolkit::path::root_dir().join("config/res").join("telegram-client.toml");
        }
        if !toml_file.exists() {
            panic!("Not found config file");
        }
        debug!("Use {:?} config file", toml_file);
        let toml = std::fs::read_to_string(toml_file).unwrap();
        Config::new(toml)
    }
}

fn get_paths(toml: &toml::Value) -> (String, String, String, String, String) {
    if let Some(table) = toml.get("paths")
        .filter(|&v| v.is_table())
        .map(|v| v.as_table())
        .filter(|&v| v.is_some())
        .map(|v| v.unwrap()) {

        let keywords = table.get("keywords").unwrap().as_str().unwrap();
        let message = table.get("message").unwrap().as_str().unwrap();
        let scammer_account = table.get("scammer-account").unwrap().as_str().unwrap();
        let about = table.get("about").unwrap().as_str().unwrap();
        let session = table.get("session").unwrap().as_str().unwrap();

        return (
            String::from(keywords),
            String::from(message),
            String::from(scammer_account),
            String::from(about),
            String::from(session)
        );
    } else {
        panic!("Missing path variables in config file");
    }
}

impl Config {

    /**
    *   For unit tests 👌
    */
    pub fn empty() -> Self {
        Config {
            toml: toml::Value::from(0),
            sender: Sender { send: false, min_wait: 0.0, max_wait: 0.0, timeout: 0.0 },
            mongo: Mongo { url: String::new() },
            keywords_path: String::new(),
            message_path: String::new(),
            scammer_account_path: String::new(),
            about_path: String::new(),
            session_path: String::new()
        }
    }

    fn new<S: AsRef<str>>(toml: S) -> Self {
        let value: toml::Value = toml::from_str(toml.as_ref()).unwrap();
        let (
            keywords_path,
            message_path,
            scammer_account_path,
            about_path,
            session_path
        ) = get_paths(&value);

        Self {
            toml: value.clone(),
            sender: get_sender(&value),
            mongo: get_mongo(&value),
            keywords_path,
            message_path,
            scammer_account_path,
            about_path,
            session_path
        }
    }

    pub fn from(path: &str) -> Self {
        let toml_file = Path::new(path);

        if !toml_file.exists() {
            panic!("Config file '{}' not found", path);
        }

        debug!("Use {:?} config file", toml_file);

        let toml = std::fs::read_to_string(toml_file).unwrap();

        Config::new(toml)
    }

    pub fn proxy(&self) -> Option<AddProxy> {
        self.toml.get("proxy")
            .filter(|&v| v.is_table())
            .map(|v| v.as_table())
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .map(|v| {
                let server = v.get("server").unwrap().as_str().unwrap();
                let port = v.get("port").unwrap().as_integer().unwrap();
                let enable = v.get("enable").unwrap().as_bool().unwrap();
                let type_ = v.get("type").unwrap().as_str().unwrap();
                let mut tga = AddProxy::builder();
                tga.server(server)
                    .port(port as i64)
                    .enable(enable);
                match type_ {
                    "socks5" => tga.type_(ProxyType::socks5(ProxyTypeSocks5::builder())),
                    "http" => tga.type_(ProxyType::http(ProxyTypeHttp::builder())),
                    "mtproto" => tga.type_(ProxyType::mtproto(ProxyTypeMtproto::builder())),
                    _ => panic!("Not found proxy type")
                };
                tga.build()
            })
    }

    pub fn log(&self) -> Option<Log> {
        self.toml.get("log")
            .filter(|&v| v.is_table())
            .map(|v| v.as_table())
            .filter(|&v| v.is_some())
            .map(|v| v.unwrap())
            .map(|v| {
                let type_ = match v.get("type").unwrap().as_str().unwrap() {
                    "console" => LogType::Console,
                    "file" => LogType::File,
                    _ => LogType::Console
                };
                let path = v.get("path").filter(|&v| v.is_str())
                    .map(|v| v.as_str())
                    .filter(|&v| v.is_some())
                    .map(|v| v.unwrap().to_string());
                let level = v.get("level").filter(|&v| v.is_integer())
                    .map(|v| v.as_integer())
                    .map(|v| v.unwrap())
                    .map_or(1, |v| v);
                Log { type_, path, level }
            })
    }

}

fn get_sender(toml: &toml::Value) -> Sender {
    toml.get("sender")
        .filter(|&v| v.is_table())
        .map(|v| v.as_table())
        .filter(|&v| v.is_some())
        .map(|v| v.unwrap())
        .map(|v| {
            let send = v.get("send").filter(|&v| v.is_bool())
                .map(|v| v.as_bool())
                .filter(|&v| v.is_some())
                .map(|v| v.unwrap())
                .unwrap();
            let min_wait = v.get("min-wait").filter(|&v| v.is_float())
                .map(|v| v.as_float())
                .filter(|&v| v.is_some())
                .map(|v| v.unwrap())
                .unwrap();
            let max_wait = v.get("max-wait").filter(|&v| v.is_float())
                .map(|v| v.as_float())
                .filter(|&v| v.is_some())
                .map(|v| v.unwrap())
                .unwrap();
            let timeout = v.get("timeout").filter(|&v| v.is_float())
                .map(|v| v.as_float())
                .filter(|&v| v.is_some())
                .map(|v| v.unwrap())
                .unwrap();

            Sender { send, min_wait, max_wait, timeout }
        })
        .unwrap()
}

fn get_mongo(toml: &toml::Value) -> Mongo {
    toml.get("mongo")
        .filter(|&v| v.is_table())
        .map(|v| v.as_table())
        .filter(|&v| v.is_some())
        .map(|v| v.unwrap())
        .map(|v| {
            let url = v.get("url").filter(|&v| v.is_str())
                .map(|v| v.as_str())
                .filter(|&v| v.is_some())
                .map(|v| v.unwrap())
                .unwrap();

            let mdb_user = urlencoding::encode(&std::env::var("MDB_USER").unwrap());
            let mdb_password = urlencoding::encode(&std::env::var("MDB_PASSWORD").unwrap());
            let mdb_address = urlencoding::encode(&std::env::var("MDB_ADDRESS").unwrap());
            let mdb_port = urlencoding::encode(&std::env::var("MDB_PORT").unwrap());

            let url = url.replace("%USERNAME%", &mdb_user);
            let url = url.replace("%PASSWORD%", &mdb_password);
            let url = url.replace("%ADDRESS%", &mdb_address);
            let url = url.replace("%PORT%", &mdb_port);

            Mongo { url }
        })
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct Log {
    pub type_: LogType,
    pub path: Option<String>,
    pub level: i64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum LogType {
    Console,
    File,
}

#[derive(Debug, Clone)]
pub struct Sender {
    // true : send message, false : don't send messages, for testing
    pub send: bool,
    // Minimum amount of seconds to wait before sending message
    pub min_wait: f64,
    // Maximum amount of seconds to wait before sending message
    pub max_wait: f64,
    // We don't answer to messages older than this timeout in seconds
    pub timeout: f64
}

#[derive(Debug, Clone)]
pub struct Mongo {
    pub url: String
}
