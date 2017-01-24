use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use portmidi;

use host::Host;
use midi::Decoder;

const MIDI_BUF_LEN: usize = 1024;
const MIDI_LOOP_DELAY_MILLIS: u64 = 10;

pub fn midi_start<'a>(pm_ctx: &'a portmidi::PortMidi, host: Arc<Mutex<Host>>) -> Result<(), portmidi::Error> {
    let in_devices: Vec<portmidi::DeviceInfo> =
        pm_ctx.devices().unwrap().into_iter().filter(|dev| dev.is_input()).collect();

    let in_ports: Vec<portmidi::InputPort> =
        in_devices.into_iter().filter_map(|dev| pm_ctx.input_port(dev, MIDI_BUF_LEN).ok()).collect();

    thread::spawn(move || {
        let loop_delay = Duration::from_millis(MIDI_LOOP_DELAY_MILLIS);
        loop {
            let mut found_events = false;
            for port in &in_ports {
                if let Ok(Some(events)) = port.read_n(MIDI_BUF_LEN) {
                    for event in events {
                        println!("[{}] {:?}", port.device(), event);
                    }
                    // tx.send((port.device(), events)).unwrap();
                    let mut locked_host = host.lock().unwrap(); // TODO What if it fails ?
                    // locked_host.proess_midi();
                    found_events = true;
                }
            }
            if !found_events {
                thread::sleep(loop_delay);
            }
        }
    });

    Ok(())
}
