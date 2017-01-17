use portaudio;

use std::sync::{Arc, Mutex};

use hero_core::types::SampleRate;
use hero_core::processing::{ProcessingArgs, Processor};

use buffers::DeinterlacedOutputBuffers;
use host::Host;

type PortAudioStream = portaudio::Stream<portaudio::NonBlocking, portaudio::Output<f32>>;

pub const SAMPLE_RATE: SampleRate = 44100 as SampleRate;
const INTERLEAVED: bool = true;
const CHANNELS: u32 = 2;
const FRAMES: u32 = 256;

pub fn audio_start<'a>(pa_ctx: &'a portaudio::PortAudio, host: Arc<Mutex<Host>>) -> Result<PortAudioStream, portaudio::error::Error> {

    let sample_rate = SAMPLE_RATE;

    //let pa_ctx = try!(portaudio::PortAudio::new());

    let default_output = try!(pa_ctx.default_output_device());
    let output_info = try!(pa_ctx.device_info(default_output));

    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    let output_params = portaudio::StreamParameters::<f32>::new(
        default_output, CHANNELS as i32, INTERLEAVED, latency);

    // Check that the stream format is supported.
    try!(pa_ctx.is_output_format_supported(output_params, sample_rate));

    // Construct the settings with which we'll open our stream.
    let settings = portaudio::OutputStreamSettings::new(
        output_params, sample_rate, FRAMES);

    let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, time, .. }| {

        let _ = time; // TODO handle time

        let mut deinterlaced = DeinterlacedOutputBuffers::from(buffer);
        let args = ProcessingArgs::new(frames, &mut deinterlaced.left, &mut deinterlaced.right);

        let mut locked_host = host.lock().unwrap(); // TODO What if it fails ?
        locked_host.process(args);

        portaudio::Continue
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = try!(pa_ctx.open_non_blocking_stream(settings, callback));
    try!(stream.start());

    Ok(stream)
}

pub fn audio_close(stream: &mut PortAudioStream) -> Result<(), portaudio::error::Error> {
    println!("Stopping and closing the stream ...");
    try!(stream.stop());
    stream.close()
}
