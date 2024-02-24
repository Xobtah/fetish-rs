use telegram_client::listener::Listener;
use log::debug;
use rtdlib::types::{SetTdlibParameters, TdlibParameters, CheckDatabaseEncryptionKey, CheckAuthenticationPassword};
use crate::{thelp, tgfn};
use std::sync::{Arc, Mutex};

pub fn auth(listener: &mut Listener, session: String) {
    let api_id = std::env::var("API_ID").unwrap();
    let api_hash = std::env::var("API_HASH").unwrap();
    let is_auth: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    listener.on_update_authorization_state(move |(api, update)| {
        let state = update.authorization_state();
        let session = session.clone();
        let api_id = api_id.clone();
        let api_hash = api_hash.clone();

        state.on_wait_tdlib_parameters(move |_| {
            api.set_tdlib_parameters(SetTdlibParameters::builder().parameters(
                TdlibParameters::builder()
                    .database_directory(session)
                    .use_message_database(true)
                    .use_secret_chats(true)
                    .api_id(toolkit::number::as_i64(api_id.clone()).unwrap())
                    .api_hash(api_hash.clone())
                    .system_language_code("ru")
                    .device_model("Desktop")
                    .system_version("Unknown")
                    .application_version(env!("CARGO_PKG_VERSION"))
                    .enable_storage_optimizer(true)
                    .build()
            ).build()).unwrap();
            debug!("Set tdlib parameters");
        });
        state.on_wait_encryption_key(|_| {
            api.check_database_encryption_key(CheckDatabaseEncryptionKey::builder().build()).unwrap();
            debug!("Set encryption key");
        });
        state.on_wait_phone_number(|_| {
            thelp::tip(format!("{} {}", "Please type your telegram phone number:", "(If you copy log to anywhere, don't forget hide your phone number)"));
            tgfn::type_phone_number(api);
        });
        state.on_wait_password(|_| {
            api.check_authentication_password(CheckAuthenticationPassword::builder()
                .password(thelp::typed_with_message(format!("{} {}", "Please type your telegram password:", "(If you copy log to anywhere, don't forget hide your password)")))
                .build()).unwrap();
            debug!("Set password *****");
        });
        state.on_wait_registration(|_| {
            thelp::tip("Welcome to use telegram");
            thelp::tip("Your phone number is not registered to telegram, please type your name. and register.");
            tgfn::type_and_register(api);
        });
        state.on_wait_code(|_| {
            thelp::tip("Please type authentication code:");
            tgfn::type_authentication_code(api);
        });

        state.on_ready(|_| {
            let mut have_authorization = is_auth.lock().unwrap();
            *have_authorization = true;
            debug!("Authorization ready");
        });
        state.on_logging_out(|_| {
            let mut have_authorization = is_auth.lock().unwrap();
            *have_authorization = false;
            debug!("Logging out");
        });
        state.on_closing(|_| {
            let mut have_authorization = is_auth.lock().unwrap();
            *have_authorization = false;
            debug!("Closing");
        });
        state.on_closed(|_| {
            debug!("Closed");
        });

        Ok(())
    });
}
