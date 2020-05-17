mod password_crypt;

use serde::{Deserialize};
use std::io;

#[derive(Deserialize)]
struct AccountInfo {
    jwt: String,
    #[serde(rename = "singleSignToken")]
    single_sign_token: String,
}

#[derive(Deserialize)]
struct Login {
    #[serde(rename = "accountInfo")]
    account_info: AccountInfo,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

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

    let client = reqwest::Client::builder()
        .user_agent(app_user_agent)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("should be able to build reqwest client");

    let resp = client.get(url.as_str())
        .send()
        .await?;
    println!("{:#?}", resp.status());
    let resp_json = resp.json::<Login>()
        .await?;
    println!("{}", resp_json.account_info.jwt);
    println!("{}", resp_json.account_info.single_sign_token);
    let url_ask_stream = format!("https://www.storytel.com/mp3streamRangeReq\
                                 ?startposition=0&programId=754932&token={}",
                                 resp_json.account_info.single_sign_token);
    println!("{}", url_ask_stream);

    let resp = client.get(&url_ask_stream)
        .send()
        .await?;

    let location = resp.headers().get("location").unwrap().to_str().unwrap();
    println!("{:#?}", resp.status());
    println!("{}", location);

    simple_example(&location.to_string());

    Ok(())

}

fn simple_example(video_path: &String) {
    let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");

    // set option "sid" to "no" (no subtitles)
    // mpv options should be set before initializing
    mpv_builder.set_option("sid","no").unwrap();

    // enable On Screen Controller (disabled with libmpv by default)
    mpv_builder.set_option("osc",true).unwrap();

    let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");

    mpv.command(&["loadfile", video_path as &str])
       .expect("Error loading file");

    // loop twice, send parameter as a string
    mpv.set_property("loop","2").unwrap();

    // set speed to 100%, send parameter as a f64
    mpv.set_property("speed",1.0).unwrap();

    // get how many loops are playing as an i64
    let n_loop : i64 = mpv.get_property("loop").unwrap() ;
    println!("NUMBER OF LOOPS IS {}",n_loop);

    'main: loop {
        while let Some(event) = mpv.wait_event(0.0) {
            // even if you don't do anything with the events, it is still necessary to empty
            // the event loop
            println!("RECEIVED EVENT : {:?}", event);
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

    println!("Simple mpv-rs example shutting down");
}
