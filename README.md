**GStreamer Video Player**

This Rust program demonstrates how to build a simple video player using GStreamer. It plays a video from a given URI and handles both audio and video streams.

**Prerequisites**

Before running the code, ensure you have the following installed:

	•	Rust (latest stable version)
	•	GStreamer (latest version)
	•	GStreamer plugins (good, bad, ugly, and base)

**Code Explanation**

The code initializes GStreamer, creates the necessary elements for the pipeline, and handles both audio and video streams from the URI.

**Key Components**

	1.	Elements:
	•	uridecodebin: Decodes media from a URI.
	•	videoconvert: Converts video formats.
	•	autovideosink: Displays video.
	•	audioconvert: Converts audio formats.
	•	audioresample: Resamples audio.
	•	autoaudiosink: Plays audio.
	2.	Pipeline:
	•	The elements are added to a pipeline and linked accordingly.
	3.	Pad Handling:
	•	The pad-added signal connects new pads from uridecodebin to the appropriate elements (audio or video).
	4.	Bus Message Handling:
	•	Handles error, state change, and EOS (end of stream) messages.

**Running the Code**

	1.	Ensure you have GStreamer installed on your system.
	2.	Add the required dependencies to your Cargo.toml.
	3.	Copy the code into your main.rs file.
	4.	Run the program using cargo run.
