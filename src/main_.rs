use std::error::Error;

extern crate rand;
use rand::Rng;

extern crate portaudio;
use portaudio::pa;

extern crate herosynth;
use herosynth::wavetable;

const SAMPLE_RATE: f64 = 44_100.0;
const CHANNELS: u32 = 2;
const FRAMES: u32 = 256;

fn main() {
    println!("PortAudio version : {}", pa::get_version());
    println!("PortAudio version text : {}", pa::get_version_text());

    match pa::initialize() {
        Ok(()) => println!("Successfully initialized PortAudio"),
        Err(err) => println!("An error occurred while initializing PortAudio: {}", err.description()),
    }

    println!("PortAudio host count : {}", pa::host::get_api_count() as isize);

    let default_host = pa::host::get_default_api();
    println!("PortAudio default host : {}", default_host as isize);

    match pa::host::get_api_info(default_host) {
        None => println!("Couldn't retrieve api info for the default host."),
        Some(info) => println!("PortAudio host name : {}", info.name),
    }

    let def_output = pa::device::get_default_output();
    let output_info = match pa::device::get_info(def_output) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving output info: {}", err.description()),
    };

    println!("Default output device name : {}", output_info.name);

    // Construct the output stream parameters.
    let output_stream_params = pa::StreamParameters {
        device : def_output,
        channel_count : CHANNELS as i32,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : output_info.default_low_output_latency
    };

    // Check that the stream format is supported.
    if let Err(err) = pa::is_format_supported(None, Some(&output_stream_params), SAMPLE_RATE) {
        panic!("The given stream format is unsupported: {:?}", err.description());
    }

    let wt = &wavetable::SAW;

    let mut o1 = 0.0f64;
    let mut o2 = 1.0f64;
    let mut o3 = 0.0f64;
    let max_offset = wavetable::SIN.size() as f64;
    let mut rng = rand::thread_rng();

    // Construct a custom callback function - in this case we're using a FnMut closure.
    let callback = Box::new(move |
        input: &[f32],
        output: &mut[f32],
        frames: u32,
        time_info: &pa::StreamCallbackTimeInfo,
        _flags: pa::StreamCallbackFlags,
        | -> pa::StreamCallbackResult {

        for sample in output.chunks_mut(CHANNELS as usize) {
            let value1 = wt.value(o1) as f32;
            let value2 = wt.value(o2) as f32;
            sample[0] = (value1 * o2 as f32 * o3 as f32) * 4.0;
            sample[1] = sample[0];
            //sample[1] = rng.gen::<f32>() * 0.2;
            //println!("output[{}]={}", i, value);

            o1 += 110.0;
            if o1 >= max_offset {
                o1 -= max_offset;
            }

            o2 -= 0.00005;
            if o2 <= 0.0 {
                o1 = 0.0; //wavetable::WT_SIN.size() as f64 / 2.0;
                o2 = 1.0;
                o3 = 0.0;
            }

            o3 += 0.005;
            if o3 >= 1.0 {
                o3 = 1.0;
            }
        }

        //println!("offset={}, frames={}", offset, frames);

        pa::StreamCallbackResult::Continue
    });

    let mut stream : pa::Stream<f32, f32> = pa::Stream::new();

    match stream.open_default(SAMPLE_RATE, FRAMES, 0, CHANNELS as i32,
                      pa::SampleFormat::Float32, Some(callback)) {
        Ok(()) => println!("Successfully opened the stream."),
        Err(err) => println!("An error occurred while opening the stream: {}", err.description()),
    }

    match stream.start() {
        Ok(()) => println!("Successfully started the stream."),
        Err(err) => println!("An error occurred while starting the stream: {}", err.description()),
    }

    // Loop while the non-blocking stream is active.
    while let Ok(true) = stream.is_active() {
        pa::sleep(1000);
    }

    match stream.close() {
        Ok(()) => println!("Successfully closed the stream."),
        Err(err) => println!("An error occurred while closing the stream: {}", err.description()),
    }

    println!("");

    match pa::terminate() {
        Ok(()) => println!("Successfully terminated PortAudio."),
        Err(err) => println!("An error occurred while terminating PortAudio: {}", err.description()),
    }
}
