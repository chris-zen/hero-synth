pub mod events;
pub mod types;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use hero_core::types::{SampleRate, Tempo, DEFAULT_TEMPO};
use hero_synth::synth::Synth as HeroSynth;

use audio::processing::{AudioOutputBuffer, ProcessingArgs, Processor};

use rosc::OscPacket;

pub use self::types::Timestamp;
pub use self::events::{Message, Event, Port, PortEvents};
use self::events::EventsBuffer;


pub struct Engine {
    sample_rate: SampleRate,
    tempo: Tempo,
    running: Arc<AtomicBool>,
    events_input_join_handler: Option<JoinHandle<()>>,
    input_events: Arc<Mutex<EventsBuffer>>,
    events_sender: Option<Sender<PortEvents>>,
    hero_synth: HeroSynth,
}

unsafe impl Send for Engine {}

impl Engine {
    pub fn new(sample_rate: SampleRate) -> Engine {
        let hero_synth = HeroSynth::new(sample_rate);

        Engine {
            sample_rate: sample_rate,
            tempo: DEFAULT_TEMPO,
            running: Arc::new(AtomicBool::new(false)),

            events_input_join_handler: None,
            input_events: Arc::new(Mutex::new(EventsBuffer::new())),
            events_sender: None,

            hero_synth: hero_synth
        }
    }

    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn start(&mut self, events_receiver: Receiver<PortEvents>, events_sender: Sender<PortEvents>) {
        let running = self.running.swap(true, Ordering::Relaxed);
        if !running {
            let running = self.running.clone();
            let input_events = self.input_events.clone();
            self.events_sender = Some(events_sender);
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
        events_receiver: Receiver<PortEvents>,
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
                        &Message::Control(ref packet) => self.hero_synth.control(packet),
                    }
                }
            }

            for sender in self.events_sender.iter() {
                let events: Vec<Event> = self.hero_synth.output().into_iter().map(|packet| {
                    Event::new(0 as Timestamp, Message::Control(packet))
                }).collect();
                if !events.is_empty() {
                    let port_events = PortEvents::new(Port::OscAll, events);
                    sender.send(port_events).ok();
                }
            };

            let (left, right) = self.hero_synth.process();
            args.audio_out_left[i] = left as f32;
            args.audio_out_right[i] = right as f32;
            process_timestamp += time_delta;
        }
    }
}
