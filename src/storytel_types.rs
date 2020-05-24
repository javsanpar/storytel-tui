use serde::{Deserialize};

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
    pub book: Book,
}

#[derive(Deserialize)]
pub struct Abook {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct Book {
    pub name: String,
}
