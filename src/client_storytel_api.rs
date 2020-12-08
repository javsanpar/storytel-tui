use crate::{mpv, password_crypt};
use serde::Deserialize;

pub struct ClientData {
    pub request_client: reqwest::blocking::Client,
    pub login_data: Login,
    pub sender: Option<std::sync::mpsc::Sender<mpv::Message>>,
    pub receiver: Option<std::sync::mpsc::Receiver<i64>>,
    pub current_abookmark_id: Option<u64>,
}

#[derive(Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "singleSignToken")]
    pub single_sign_token: String,
}

#[derive(Deserialize)]
pub struct Login {
    #[serde(rename = "accountInfo")]
    pub account_info: AccountInfo,
}

#[derive(Deserialize)]
pub struct BookShelf {
    #[serde(rename = "books")]
    pub books: Vec<BookEntry>,
}

#[derive(Deserialize)]
pub struct BookEntry {
    pub abook: Option<Abook>,
    #[serde(rename = "abookMark")]
    pub abookmark: Option<AbookMark>,
    pub book: Book,
}

#[derive(Deserialize)]
pub struct AbookMark {
    #[serde(rename = "bookId")]
    pub id: u64,
    #[serde(rename = "pos")]
    pub position: i64,
}

#[derive(Deserialize)]
pub struct Abook {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct Book {
    pub name: String,
}

pub fn login(client_data: &mut ClientData, email: &str, pass: &str) {
    let hex_encryp_pass = password_crypt::encrypt_password(&pass.trim());

    let url = format!(
        "https://www.storytel.com/api/login.action\
                      ?m=1&uid={}&pwd={}",
        email.trim(),
        hex_encryp_pass
    );

    let resp_login = client_data.request_client.get(&url).send();

    client_data.login_data = resp_login.unwrap().json::<Login>().unwrap()
}

pub fn get_bookshelf(client_data: &mut ClientData) -> BookShelf {
    let url_get_bookshelf = format!(
        "https://www.storytel.com/api/getBookShelf.\
                                    action?token={}",
        client_data.login_data.account_info.single_sign_token
    );
    let resp_bookshelf = client_data.request_client.get(&url_get_bookshelf).send();

    resp_bookshelf.unwrap().json::<BookShelf>().unwrap()
}

pub fn get_stream_url(client_data: &mut ClientData, id: &u64) -> String {
    let url_ask_stream = format!(
        "https://www.storytel.com/mp3streamRangeReq\
                                 ?startposition=0&programId={}&token={}",
        id, client_data.login_data.account_info.single_sign_token
    );

    let resp = client_data.request_client.get(&url_ask_stream).send();

    resp.as_ref()
        .unwrap()
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn set_bookmark(client_data: &mut ClientData, position: i64) {
    let microsec_to_sec = 1000000;
    let params = [
        (
            "token",
            client_data
                .login_data
                .account_info
                .single_sign_token
                .to_string(),
        ),
        (
            "bookId",
            client_data.current_abookmark_id.unwrap().to_string(),
        ),
        ("pos", (position * microsec_to_sec).to_string()),
        ("type", "1".to_string()),
    ];
    let url_set_bookmark = format!("https://www.storytel.se/api/setABookmark.action");
    client_data
        .request_client
        .post(&url_set_bookmark)
        .form(&params)
        .send()
        .unwrap();
}
