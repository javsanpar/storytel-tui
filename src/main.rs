mod password_crypt;
mod storytel_types;
mod mpv;

use std::io;




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
    let login = resp_login.unwrap().json::<storytel_types::Login>().unwrap();

    let url_get_bookshelf = format!("https://www.storytel.com/api/getBookShelf.\
                                    action?token={}",
                                    login.account_info.single_sign_token);
    let resp_bookshelf = client.get(&url_get_bookshelf)
        .send();
    let bookshelf = resp_bookshelf.unwrap().json::<storytel_types::BookShelf>()
        .unwrap();
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

    mpv::simple_example(location);


}
