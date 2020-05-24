pub fn simple_example(video_path: &str) {
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
