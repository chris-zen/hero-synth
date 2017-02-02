use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::mpsc::{Sender, Receiver};
use std::thread::{self, JoinHandle};

use rosc;

use midi;
use engine;
use engine::Timestamp;

pub struct Control {
    running: Arc<AtomicBool>,
    midi_join_handler: Option<JoinHandle<()>>,
    osc_join_handler: Option<JoinHandle<()>>,
}

impl Control {
    pub fn new() -> Self {
        Control {
            running: Arc::new(AtomicBool::new(false)),
            midi_join_handler: None,
            osc_join_handler: None,
        }
    }

    pub fn start(&mut self,
                 midi_input_rx: Receiver<midi::PortEvents>,
                 osc_input_rx: Receiver<rosc::OscPacket>,
                 host_events_tx: Sender<engine::PortEvents>) {

        let midi_events_tx = host_events_tx.clone();
        self.midi_join_handler = Some(thread::spawn(move || {
            Control::midi_input(midi_input_rx, midi_events_tx) }));

        let osc_events_tx = host_events_tx.clone();
        self.osc_join_handler = Some(thread::spawn(move || {
            Control::osc_input(osc_input_rx, osc_events_tx) }));
    }

    pub fn stop(&mut self) {
        let running = self.running.swap(false, Ordering::Relaxed);
        if running {
            self.midi_join_handler.take().unwrap().join().ok();
        }
    }

    fn midi_input(midi_input_rx: Receiver<midi::PortEvents>,
                  host_events_tx: Sender<engine::PortEvents>) {

        for midi_port_events in midi_input_rx {
            let mut engine_events: Vec<engine::Event> = Vec::new();
            for midi_event in midi_port_events.events() {
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
            let device = engine::events::Port::Midi(midi_port_events.port().to_string());
            let engine_src_events = engine::PortEvents::new(device, engine_events);
            host_events_tx.send(engine_src_events).unwrap();
        }
    }

    fn osc_input(osc_input_rx: Receiver<rosc::OscPacket>,
                  host_events_tx: Sender<engine::PortEvents>) {

        const NOW_TIMESTAMP: Timestamp = 0 as Timestamp;
        let default_port: engine::Port = engine::Port::Osc("default".to_string());

        for osc_packet in osc_input_rx {
            let event = engine::Event::new(NOW_TIMESTAMP, engine::Message::Control(osc_packet));
            let src_events = engine::PortEvents::new(default_port.clone(), vec![event]);
            host_events_tx.send(src_events).unwrap();
        }
    }
}
