# GStreamer RTMP Video Player

This Rust program demonstrates how to build a simple RTMP player using GStreamer. It plays a video from a given RTMP (or any) URI and handles both audio and video streams.

**Prerequisites**

Before running the code, ensure you have the following installed:

 - Rust (latest stable version)
 - GStreamer (latest version)
 - GStreamer plugins (good, bad, ugly, and base)

**Code Explanation**

The code initializes GStreamer, creates the necessary elements for the pipeline, and handles both audio and video streams from the URI.

**Key Components**

*Elements:*
 - uridecodebin: Decodes media from a URI.
 - videoconvert: Converts video formats.
 - autovideosink: Displays video.
 - audioconvert: Converts audio formats.
 - audioresample: Resamples audio.
 - autoaudiosink: Plays audio.
   
*Pipeline:*
 - The elements are added to a pipeline and linked accordingly.
   
*Pad Handling:*
 - The pad-added signal connects new pads from uridecodebin to the appropriate elements (audio or video).
   
*Bus Message Handling:*
 - Handles error, state change, and EOS (end of stream) messages.

**Running the Code**
 - Ensure you have GStreamer installed on your system.
 - Add the required dependencies to your Cargo.toml.
 - Copy the code into your main.rs file.
 - Run the program using cargo run.
