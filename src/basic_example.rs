use gst::prelude::*;
use gstreamer::{self as gst, caps::SomeFeatures};

fn tutorial_main() {
    // Initialize GStreamer
    gst::init().unwrap();

    // Build the pipeline
    // Create the empty pipeline
    let pipeline = gst::Pipeline::with_name("test-pipeline");

    println!("Random Random Ranom");

    // Create the elements
    let source = gst::ElementFactory::make("videotestsrc")
        .name("source")
        .property_from_str("pattern", "smpte")
        .build()
        .expect("Could not create source element.");

    let sink = gst::ElementFactory::make("autovideosink")
        .name("sink")
        .build()
        .expect("Could not create sink element");

    pipeline.add_many([&source, &sink]).unwrap();
    source.link(&sink).expect("Elements could not be linked.");

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
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

}
