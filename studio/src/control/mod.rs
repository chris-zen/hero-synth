use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};

use midi;
use engine;

pub struct Control {
    running: Arc<AtomicBool>,
    join_handler: Option<JoinHandle<()>>,
}

impl Control {
    pub fn new() -> Self {
        Control {
            running: Arc::new(AtomicBool::new(false)),
            join_handler: None,
        }
    }

    pub fn start(&mut self,
                 midi_input_rx: Receiver<midi::DeviceEvents>,
                 host_notes_tx: Sender<engine::DeviceEvents>) {

        self.join_handler = Some(thread::spawn(move || {
            for midi_dev_events in midi_input_rx {
                let mut engine_events: Vec<engine::Event> = Vec::new();
                for midi_event in midi_dev_events.events() {
                    match midi_event.message() {
                        midi::Message::NoteOn { key, velocity, .. } => {
                            let velocity = velocity as f64 / 127.0;
                            let engine_message = engine::Message::NoteOn {key: key, velocity: velocity };
                            let engine_event = engine::Event::new(midi_event.timestamp(), engine_message);
                            engine_events.push(engine_event);
                        },
                        midi::Message::NoteOff { key, velocity, .. } => {
                            let velocity = velocity as f64 / 127.0;
                            let engine_message = engine::Message::NoteOff {key: key, velocity: velocity };
                            let engine_event = engine::Event::new(midi_event.timestamp(), engine_message);
                            engine_events.push(engine_event);
                        },
                        _ => {}
                    }
                }
                let engine_dev_events = engine::DeviceEvents::new(midi_dev_events.device(), engine_events);
                host_notes_tx.send(engine_dev_events).unwrap();
            }
        }));
    }

    pub fn stop(&mut self) {
        let running = self.running.swap(false, Ordering::Relaxed);
        if running {
            self.join_handler.take().unwrap().join().ok();
        }
    }
}
