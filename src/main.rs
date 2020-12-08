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

    siv.set_user_data(client_data);

    tui::show_login(&mut siv);

    siv.run();
}
