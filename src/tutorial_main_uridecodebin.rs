use gst::prelude::*;
use gstreamer::{self as gst, caps::SomeFeatures};

pub fn tutorial_main_uridecode() {
    // Initialize GStreamer
    gst::init().unwrap();

    let uri = "https://live-par-2-cdn-alt.livepush.io/live/bigbuckbunnyclip/index.m3u8";

    // Create the elements
    let source = gst::ElementFactory::make("uridecodebin")
        .name("source")
        .property("uri", uri)
        .build()
        .expect("Could not create uridecodebin element.");
    let test_source = gst::ElementFactory::make("videotestsrc")
        .name("test_source")
        .build()
        .expect("Could not create videotestsrc element.");
    let video_convert1 = gst::ElementFactory::make("videoconvert")
        .name("video_convert1")
        .build()
        .expect("Could not create videoconvert element.");
    let video_convert2 = gst::ElementFactory::make("videoconvert")
        .name("video_convert2")
        .build()
        .expect("Could not create videoconvert element.");
    let video_sink = gst::ElementFactory::make("autovideosink")
        .name("video_sink")
        .build()
        .expect("Could not create autovideosink element.");
    let compositor = gst::ElementFactory::make("compositor")
        .name("compositor")
        .build()
        .expect("Could not create compositor element.");
    let audio_convert = gst::ElementFactory::make("audioconvert")
        .name("audio_convert")
        .build()
        .expect("Could not create audioconvert element.");
    let audio_resample = gst::ElementFactory::make("audioresample")
        .name("audio_resample")
        .build()
        .expect("Could not create audioresample element.");
    let audio_sink = gst::ElementFactory::make("autoaudiosink")
        .name("audio_sink")
        .build()
        .expect("Could not create autoaudiosink element.");

    // Create the empty pipeline
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    // Set properties for the test video source
    test_source.set_property_from_str("pattern", "smpte");

    // Build the pipeline
    pipeline
        .add_many(&[
            &source,
            &test_source,
            &video_convert1,
            &video_convert2,
            &compositor,
            &video_sink,
            &audio_convert,
            &audio_resample,
            &audio_sink,
        ])
        .unwrap();
    gst::Element::link_many(&[&video_convert1, &compositor])
        .expect("Video elements could not be linked.");
    gst::Element::link_many(&[&video_convert2, &compositor])
        .expect("Video elements could not be linked.");
    gst::Element::link_many(&[&compositor, &video_sink])
        .expect("Compositor to video sink could not be linked.");
    gst::Element::link_many(&[&audio_convert, &audio_resample, &audio_sink])
        .expect("Audio elements could not be linked.");

    // Link the test source to the compositor
    test_source
        .link(&video_convert2)
        .expect("Test source could not be linked to video convert");

    // Connect the pad-added signal
    source.connect_pad_added(move |src, src_pad| {
        println!("Received new pad {} from {}", src_pad.name(), src.name());

        let new_pad_caps = src_pad
            .current_caps()
            .expect("Failed to get caps of new pad.");
        let new_pad_struct = new_pad_caps
            .structure(0)
            .expect("Failed to get first structure of caps.");
        let new_pad_type = new_pad_struct.name();

        if new_pad_type.starts_with("video/x-raw") {
            let sink_pad = video_convert1
                .static_pad("sink")
                .expect("Failed to get static sink pad from video_convert1");
            if sink_pad.is_linked() {
                println!("Video pad is already linked. Ignoring.");
                return;
            }

            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Type is {new_pad_type} but link failed.");
            } else {
                println!("Link succeeded (type {new_pad_type}).");
            }
        } else if new_pad_type.starts_with("audio/x-raw") {
            let sink_pad = audio_convert
                .static_pad("sink")
                .expect("Failed to get static sink pad from audio_convert");
            if sink_pad.is_linked() {
                println!("Audio pad is already linked. Ignoring.");
                return;
            }

            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Type is {new_pad_type} but link failed.");
            } else {
                println!("Link succeeded (type {new_pad_type}).");
            }
        } else {
            println!("It has type {new_pad_type} which is not raw audio or video. Ignoring.");
        }
    });

    // Start playing
    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait until error or EOS
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed.src().map(|s| s == &pipeline).unwrap_or(false) {
                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        state_changed.old(),
                        state_changed.current()
                    );
                }
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
