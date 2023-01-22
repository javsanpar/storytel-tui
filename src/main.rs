use cursive::Cursive;
use cursive::reexports::log;
use cursive::reexports::log::LevelFilter;

mod client_storytel_api;
mod mpv;
mod password_crypt;
mod tui;

fn main() {
    let user_agent: &str = "okhttp/3.12.8";
    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("should be able to build reqwest client");
    let login_data = client_storytel_api::Login {
        account_info: client_storytel_api::AccountInfo {
            single_sign_token: String::from(""),
        },
    };
    let client_data = client_storytel_api::ClientData {
        request_client: client,
        login_data: login_data,
        sender: None,
        receiver: None,
        current_abookmark_id: None,
    };

    let mut siv = cursive::default();

    cursive::logger::init();
    match std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()).as_ref() {
        "trace" => log::set_max_level(LevelFilter::Trace),
        "debug" => log::set_max_level(LevelFilter::Debug),
        "info" => log::set_max_level(LevelFilter::Info),
        "warn" => log::set_max_level(LevelFilter::Warn),
        "error" => log::set_max_level(LevelFilter::Error),
        _ => log::set_max_level(LevelFilter::Off),
    }
    siv.add_global_callback('~', Cursive::toggle_debug_console);
    siv.set_user_data(client_data);

    tui::show_login(&mut siv);

    siv.run();
}
