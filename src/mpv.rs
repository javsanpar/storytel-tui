use crate::client_storytel_api;
use cursive::Cursive;
use std::sync::mpsc;
use cursive::reexports::log;

pub enum Message {
    Play,
    Pause,
    Forward,
    Backward,
    Speed{
        value:i32
    },
}

pub fn speed(siv: &mut Cursive, am: &i32)  {
     send_message(siv, Message::Speed{value: *am });
}

pub fn play(siv: &mut Cursive) {
    send_message(siv, Message::Play);
}

pub fn pause(siv: &mut Cursive) {
    send_message(siv, Message::Pause);

    let client_data = siv.user_data::<client_storytel_api::ClientData>().unwrap();
    let position = client_data.receiver.as_ref().unwrap().recv().unwrap();
    client_storytel_api::set_bookmark(client_data, position);
}

pub fn forward(siv: &mut Cursive) {
    send_message(siv, Message::Forward);
}

pub fn backward(siv: &mut Cursive) {
    send_message(siv, Message::Backward);
}

fn send_message(siv: &mut Cursive, message: Message) {
    let client_data = siv.user_data::<client_storytel_api::ClientData>().unwrap();

    if let Some(sender) = client_data.sender.as_ref() {
        sender.send(message).unwrap();
    }
}

pub fn simple_example(
    video_path: String,
    position: i64,
) -> (
    mpsc::Sender<Message>,
    mpsc::Receiver<i64>,
) {
    let (sender_tui, receiver_mpv) = mpsc::channel();
    let (sender_mpv, receiver_tui) = mpsc::channel();
    std::thread::spawn(move || {
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");

        // set option "sid" to "no" (no subtitles)
        // mpv options should be set before initializing
        mpv_builder.set_option("sid", "no").unwrap();

        // enable On Screen Controller (disabled with libmpv by default)
        mpv_builder.set_option("osc", true).unwrap();
        mpv_builder.set_option("log-file", "/tmp/mpv.log").unwrap(); //XXX

        mpv_builder
            .set_option("start", position.to_string().as_str())
            .unwrap();

        let mut mpv = mpv_builder.build().expect("Failed to build MPV handler");

        mpv.command(&["loadfile", video_path.as_str()])
            .expect("Error loading file");

        // loop twice, send parameter as a string
        mpv.set_property("loop", "2").unwrap();

        // set speed to 100%, send parameter as a f64
        mpv.set_property("speed", 1.0).unwrap();

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
                match receiver_mpv.recv() {
                    Ok(Message::Play) => mpv
                        .set_property_async("pause", false, 1)
                        .expect("Error setting MPV property"),
                    Ok(Message::Pause) => {
                        mpv.set_property_async("pause", true, 1)
                            .expect("Error setting MPV property");
                        let position: i64 = mpv.get_property("time-pos").unwrap();
                        sender_mpv.send(position).unwrap();
                    }
                    Ok(Message::Forward) => mpv
                        .command(&["seek", "5"])
                        .expect("Error setting MPV property"),
                    Ok(Message::Backward) => mpv
                        .command(&["seek", "-5"])
                        .expect("Error setting MPV property"),
                    Ok(Message::Speed{value}) => {
                        let speed_val = value as f64 / 100f64;
                        log::info!("item:{}, speed:{}", value, speed_val);
                        if speed_val > 0.1 {
                            mpv
                                .set_property("speed", speed_val)
                                .expect("Error setting MPV property");
                        }
                    }
                    _ => break 'main,
                };
            }
        }
    });

    return (sender_tui, receiver_tui);
}
