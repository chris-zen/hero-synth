use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering, AtomicBool};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use portmidi;

use midi::decoder::Decoder;
use midi::events::{Event, PortEvents};
use midi::types::Timestamp;

const MIDI_BUF_LEN: usize = 1024;
const MIDI_LOOP_DELAY_MILLIS: u64 = 10;

pub struct Midi {
    pm_ctx: portmidi::PortMidi,
    running: Arc<AtomicBool>,
    finished: Arc<AtomicBool>
}

impl Midi {
    pub fn new() -> Midi {
        let pm_ctx = portmidi::PortMidi::new().unwrap();
        Midi {
            pm_ctx: pm_ctx,
            running: Arc::new(AtomicBool::new(false)),
            finished: Arc::new(AtomicBool::new(true))
        }
    }

    pub fn start(&mut self, sender: Sender<PortEvents>/*, receiver: Receiver<Vec<Event>>*/) {
        let running = self.running.swap(true, Ordering::Relaxed);
        if !running {
            let in_devices: Vec<portmidi::DeviceInfo> =
                self.pm_ctx.devices().unwrap().into_iter().filter(|dev| dev.is_input()).collect();

            let in_ports: Vec<portmidi::InputPort> =
                in_devices.into_iter().filter_map(|dev| self.pm_ctx.input_port(dev, MIDI_BUF_LEN).ok()).collect();

            let running = self.running.clone();
            let finished = self.finished.clone();

            thread::spawn(move || Self::read_loop(&in_ports, &running, &finished, sender));
        }
    }

    pub fn stop(&mut self) {
        let running = self.running.swap(false, Ordering::Relaxed);
        if running {
            let wait = Duration::from_millis(250);
            while !self.finished.load(Ordering::Relaxed) {
                thread::sleep(wait);
            }
        }
    }

    fn read_loop(in_ports: &Vec<portmidi::InputPort>,
                 running: &AtomicBool, finished: &AtomicBool,
                 sender: Sender<PortEvents>) {

        finished.store(false, Ordering::Relaxed);
        let loop_delay = Duration::from_millis(MIDI_LOOP_DELAY_MILLIS);
        let mut dev_events = Vec::<PortEvents>::with_capacity(in_ports.len());
        while running.load(Ordering::Relaxed) {
            Self::read_events(&in_ports, &mut dev_events);
            if !dev_events.is_empty() {
                for dev_events in dev_events.iter() {
                    sender.send(dev_events.clone()).ok();
                }
            }
            else {
                thread::sleep(loop_delay);
            }
        }
        finished.store(true, Ordering::Relaxed);
    }

    fn read_events(in_ports: &Vec<portmidi::InputPort>, dev_events: &mut Vec<PortEvents>) {
        dev_events.clear();
        for port in in_ports {
            if let Ok(Some(raw_events)) = port.read_n(MIDI_BUF_LEN) {
                let events = Self::decode_events(raw_events);
                if !events.is_empty() {
                    let device = port.device();
                    let dev_name = device.name();
                    dev_events.push(PortEvents::new(dev_name, events));
                }
            }
        }
    }

    fn decode_events(raw_events: Vec<portmidi::MidiEvent>) -> Vec<Event> {
        let mut events = Vec::with_capacity(raw_events.len());
        for raw_event in raw_events {
            let raw_msg = raw_event.message;
            let msg_buf = [raw_msg.status, raw_msg.data1, raw_msg.data2];
            let mut decoder = Decoder::new(&msg_buf);
            match decoder.next() {
                Some(message) => {
                    let timestamp = raw_event.timestamp as Timestamp;
                    let event = Event::new(timestamp, message);
                    events.push(event);
                },
                None => {}
            }
        }
        events.sort_by_key(|event| event.timestamp());
        events
    }
}
