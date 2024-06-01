#[path = "common.rs"]
mod common;

#[path = "gst_rtmp.rs"]
mod gst_rtmp;

fn main() {
    //common::run(tutorial_main); // uncomment if you want to run some basic example
    common::run(gst_rtmp::tutorial_main_uridecode);
}
