pub mod events;
pub mod types;

// use std::cmp::Ordering as O;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use hero_core::types::{SampleRate, Tempo, DEFAULT_TEMPO};
use hero_synth::synth::Synth as HeroSynth;

use audio;
use audio::processing::{AudioOutputBuffer, ProcessingArgs, Processor};

use self::types::Timestamp;
use self::events::EventsBuffer;
pub use self::events::{Event, DeviceEvents};
pub use self::events::Message;

// impl Ord for midi::Event {
//     fn cmp(&self, other: &midi::Event) -> O {
//         other.timestamp().cmp(&self.timestamp())
//     }
// }
//
// impl PartialOrd for midi::Event {
//     fn partial_cmp(&self, other: &midi::Event) -> Option<O> {
//         Some(self.cmp(other))
//     }
// }


pub struct Engine {
    sample_rate: f64,
    tempo: f64,
    running: Arc<AtomicBool>,
    events_input_join_handler: Option<JoinHandle<()>>,
    input_events: Arc<Mutex<EventsBuffer>>,
    // output_events_sender: Sender<DeviceEvents>
    hero_synth: HeroSynth
}

unsafe impl Send for Engine {}

impl Engine {
    pub fn new(sample_rate: SampleRate/*, events_sender: Sender<DeviceEvents>*/) -> Engine {
        let mut hero_synth = HeroSynth::new(sample_rate);
        // hero_synth.note_on(33, 1.0);
        // hero_synth.note_on(81, 0.5);

        Engine {
            sample_rate: sample_rate,
            tempo: DEFAULT_TEMPO,
            running: Arc::new(AtomicBool::new(false)),

            events_input_join_handler: None,
            input_events: Arc::new(Mutex::new(EventsBuffer::new())),
            // output_events_sender: events_sender

            hero_synth: hero_synth
        }
    }

    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn start(&mut self, events_receiver: Receiver<DeviceEvents>) {
        let running = self.running.swap(true, Ordering::Relaxed);
        if !running {
            let running = self.running.clone();
            let input_events = self.input_events.clone();
            self.events_input_join_handler = Some(thread::spawn(move || {
                Self::events_input_loop(&running, events_receiver, input_events)
            }));
        }
    }

    pub fn stop(&mut self) {
        let running = self.running.swap(false, Ordering::Relaxed);
        if running {
            self.events_input_join_handler.take().unwrap().join().ok();
        }
    }

    fn events_input_loop(
        running: &AtomicBool,
        events_receiver: Receiver<DeviceEvents>,
        input_events_mutex: Arc<Mutex<EventsBuffer>>) {

        while running.load(Ordering::Relaxed) {
            for dev_events in events_receiver.iter() {
                let mut input_events = input_events_mutex.lock().unwrap();
                for event in dev_events.events() {
                    input_events.push(event);
                    println!("{:?}", event);
                }
            }
        }
    }
}

impl<'a, O> Processor<'a, f32, O> for Engine
    where O: AudioOutputBuffer<Output=f32> {

    fn process(&mut self, args: ProcessingArgs<'a, f32, O>) {
        let timestamp = args.timestamp;
        let time_delta = 1000000000.0 / self.sample_rate;
        let duration = (args.num_frames as f64 * time_delta).ceil() as Timestamp;
        let mut process_timestamp = timestamp as f64;

        let mut frame_events = { self.input_events.lock().unwrap().split(timestamp + duration) };

        for i in 0..args.num_frames {
            let next_timestamp = (process_timestamp + time_delta).ceil() as Timestamp;
            let proc_events = frame_events.split(next_timestamp);
            for (_timestamp, messages) in proc_events.iter() {
                for message in messages.iter() {
                    match message {
                        &Message::NoteOn { key, velocity } => self.hero_synth.note_on(key, velocity),
                        &Message::NoteOff { key, velocity } => self.hero_synth.note_off(key, velocity),
                    }
                }
            }
            let (left, right) = self.hero_synth.process();
            args.audio_out_left[i] = left as f32;
            args.audio_out_right[i] = right as f32;
            process_timestamp += time_delta;
        }
    }
}
