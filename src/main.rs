use std::error::Error;
use ::std::f64::consts::PI;

extern crate rand;
use rand::Rng;

extern crate portaudio;
use portaudio::pa;

extern crate herosynth;
use herosynth::wavetable;
use herosynth::oscillator::Oscillator;

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

    let wt = &wavetable::SIN;

    let fc = 220.0f64;
    let fm1 = 4.0f64;
    let fm2 = 220.0f64;
    let mi = 6.0;

    let mut oc1 = Oscillator::new(SAMPLE_RATE, wt);
    oc1.set_base_frequency(fc);

    let mut om1 = Oscillator::new(SAMPLE_RATE, wt);
    om1.set_amplitude(mi * fm1);
    om1.set_base_frequency(fm1);

    let mut om2 = Oscillator::new(SAMPLE_RATE, wt);
    om2.set_amplitude(mi * fm2);
    om2.set_base_frequency(fm2);

    // Construct a custom callback function - in this case we're using a FnMut closure.
    let callback = Box::new(move |
        input: &[f32],
        output: &mut[f32],
        frames: u32,
        time_info: &pa::StreamCallbackTimeInfo,
        _flags: pa::StreamCallbackFlags,
        | -> pa::StreamCallbackResult {

        for sample in output.chunks_mut(CHANNELS as usize) {
            let cs = oc1.process() as f32;
            let ms1 = om1.process();
            let ms2 = om2.process();
            //oc1.set_amplitude(value2);
            oc1.set_phase_modulation(ms1 + ms2);

            sample[0] = cs;
            sample[1] = cs;
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
