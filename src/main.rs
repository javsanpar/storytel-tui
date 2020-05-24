mod password_crypt;

use serde::{Deserialize};
use std::io;

#[derive(Deserialize)]
struct AccountInfo {
    #[serde(rename = "singleSignToken")]
    single_sign_token: String,
}

#[derive(Deserialize)]
struct Login {
    #[serde(rename = "accountInfo")]
    account_info: AccountInfo,
}

#[derive(Deserialize)]
struct BookShelf {
    #[serde(rename = "books")]
    books: Vec<BookEntry>,
}

#[derive(Deserialize)]
struct BookEntry {
    abook: Option<Abook>,
    book: Book,
}

#[derive(Deserialize)]
struct Abook {
    id: u64,
}

#[derive(Deserialize)]
struct Book {
    name: String,
}

fn main() {

    println!("Please input your email");
    let mut email = String::new();
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read email");

    println!("Please input your pass");
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read pass");

    let app_user_agent: &str = "okhttp/3.12.8";
    let hex_encryp_pass = password_crypt::encrypt_password(&pass.trim());

    let url = format!("https://www.storytel.com/api/login.action\
                      ?m=1&uid={}&pwd={}",
                      email.trim(), hex_encryp_pass);

    let client = reqwest::blocking::Client::builder()
        .user_agent(app_user_agent)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("should be able to build reqwest client");

    let resp_login = client.get(&url)
        .send();
    let login = resp_login.unwrap().json::<Login>().unwrap();

    let url_get_bookshelf = format!("https://www.storytel.com/api/getBookShelf.\
                                    action?token={}",
                                    login.account_info.single_sign_token);
    let resp_bookshelf = client.get(&url_get_bookshelf)
        .send();
    let bookshelf = resp_bookshelf.unwrap().json::<BookShelf>().unwrap();
    for (i, bookentry) in bookshelf.books.iter().enumerate() {
        match &bookentry.abook {
            Some(abook) => println!("Index: {}\n{}", i, abook.id),
            None    => continue,
        }
        println!("{}", bookentry.book.name);
    }

    println!("Please input desired index");
    let mut index_string = String::new();
    io::stdin()
        .read_line(&mut index_string)
        .expect("Failed to read index");
    let index:usize = match index_string.trim().parse::<usize>() {
        Ok(i) => i,
        Err(..) => 0,
    };

    let id = bookshelf.books[index].abook.as_ref().unwrap().id;

    let url_ask_stream = format!("https://www.storytel.com/mp3streamRangeReq\
                                 ?startposition=0&programId={}&token={}",
                                 id,
                                 login.account_info.single_sign_token);

    let resp = client.get(&url_ask_stream)
        .send();

    let location = resp.as_ref().unwrap().headers().get("location").unwrap()
        .to_str().unwrap();

    simple_example(location);


}

fn simple_example(video_path: &str) {
    let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");

    // set option "sid" to "no" (no subtitles)
    // mpv options should be set before initializing
    mpv_builder.set_option("sid","no").unwrap();

    // enable On Screen Controller (disabled with libmpv by default)
    mpv_builder.set_option("osc",true).unwrap();

    let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");

    mpv.command(&["loadfile", video_path])
       .expect("Error loading file");

    // loop twice, send parameter as a string
    mpv.set_property("loop","2").unwrap();

    // set speed to 100%, send parameter as a f64
    mpv.set_property("speed",1.0).unwrap();

    'main: loop {
        while let Some(event) = mpv.wait_event(0.0) {
            // even if you don't do anything with the events, it is still necessary to empty
            // the event loop
            match event {
                // Shutdown will be triggered when the window is explicitely closed,
                // while Idle will be triggered when the queue will end
                mpv::Event::Shutdown | mpv::Event::Idle => {
                    break 'main;
                }
                _ => {}
            };
        }
    }

}
