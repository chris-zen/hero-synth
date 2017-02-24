use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

use rosc::{OscType, OscMessage, OscBundle, OscPacket};

use hero_core::types::{SampleRate, DEFAULT_SAMPLE_RATE};

use patch::Patch;
use voice::{Voice, MAX_OSCILLATORS};

const MAX_KEYS: usize = 128;

const ADDR_SYNC: &'static str = "/sync";
const ADDR_NOTE: &'static str = "/note";
const ADDR_OSC_ENABLED: &'static str = "/osc/enabled";
const ADDR_OSC_FREE_PHASE: &'static str = "/osc/free-phase";
const ADDR_OSC_INITIAL_PHASE: &'static str = "/osc/phase";
const ADDR_OSC_AMP: &'static str = "/osc/amp";
const ADDR_OSC_FIXED_FREQ: &'static str = "/osc/fixed-freq";
const ADDR_OSC_FREQ: &'static str = "/osc/freq";
const ADDR_OSC_OCTAVES: &'static str = "/osc/octaves";
const ADDR_OSC_SEMITONES: &'static str = "/osc/semitones";
const ADDR_OSC_DETUNE: &'static str = "/osc/detune";
const ADDR_OSC_AMP_MOD: &'static str = "/osc/am";
const ADDR_OSC_FREQ_MOD: &'static str = "/osc/fm";
const ADDR_OSC_PAN: &'static str = "/osc/pan";
const ADDR_OSC_LEVEL: &'static str = "/osc/level";

pub struct Synth {
    sample_rate: SampleRate,
    patch: Rc<RefCell<Patch>>,
    patch_version: usize,
    voices: Vec<Voice>,
    active_voices: HashSet<usize>,
    output_packets: Vec<OscPacket>,
}

impl Default for Synth {
    fn default() -> Self {
        Synth {
            sample_rate: DEFAULT_SAMPLE_RATE,
            patch: Rc::new(RefCell::new(Patch::default())),
            patch_version: 0,
            voices: Vec::new(),
            active_voices: HashSet::new(),
            output_packets: Vec::new(),
        }
    }
}

impl Synth {
    pub fn new(sample_rate: SampleRate) -> Synth {
        let patch = Rc::new(RefCell::new(Patch::default()));
        let mut voices = Vec::<Voice>::with_capacity(MAX_KEYS);
        for _key in 0..MAX_KEYS {
            let voice = Voice::new(sample_rate, patch.clone());
            voices.push(voice);
        }

        Synth {
            sample_rate: sample_rate,
            patch: patch,
            voices: voices,

            ..Synth::default()
        }
    }

    pub fn get_sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn note_on(&mut self, key: usize, vel: f64) {
        let voice_index = key & 0x7f;
        let ref mut voice = self.voices[voice_index];
        voice.reset();
        voice.update_patch(&self.patch.borrow(), self.patch_version);
        voice.note_on(key, vel);
        self.active_voices.insert(voice_index);
    }

    pub fn note_off(&mut self, key: usize, vel: f64) {
        let voice_index = key & 0x7f;
        let ref mut voice = self.voices[voice_index];
        voice.note_off(key, vel);
        self.active_voices.remove(&voice_index);
    }

    pub fn control(&mut self, packet: &OscPacket) {
        match packet {
            &OscPacket::Message(ref msg) => {
                match msg.addr.as_ref() {
                    ADDR_SYNC => self.control_sync(&msg.args),
                    ADDR_NOTE => self.control_note(&msg.args),
                    ADDR_OSC_AMP => self.control_osc_amplitude(&msg.args),
                    ADDR_OSC_FREQ => self.control_osc_frequency(&msg.args),
                    ADDR_OSC_OCTAVES => self.control_osc_octaves(&msg.args),
                    ADDR_OSC_SEMITONES => self.control_osc_semitones(&msg.args),
                    ADDR_OSC_LEVEL => self.control_osc_level(&msg.args),
                    ADDR_OSC_PAN => self.control_osc_panning(&msg.args),
                    ADDR_OSC_FREQ_MOD => self.control_osc_fm_mod(&msg.args),
                    ADDR_OSC_ENABLED => self.control_osc_enabled(&msg.args),
                    ADDR_OSC_FIXED_FREQ => self.control_osc_fixed_freq(&msg.args),
                    ADDR_OSC_FREE_PHASE => self.control_osc_free_phase(&msg.args),
                    _ => {}
                }
            },
            &OscPacket::Bundle(ref bundle) => {
                for bundle_packet in &bundle.content {
                    self.control(&bundle_packet);
                }
            }
        }
    }

    pub fn output(&mut self) -> Vec<OscPacket> {
        let packets = self.output_packets.to_owned();
        self.output_packets = Vec::new();
        packets
    }

    fn control_sync(&mut self, _args: &Option<Vec<OscType>>) {
        let mut packets = Vec::with_capacity(8 * (11 + 8 * 2));
        use rosc::OscType::{Int, Float, Time};
        let patch = self.patch.borrow();
        for i in 0..patch.oscillators.len() {
            let index = i as i32;
            let patch_osc = &patch.oscillators[i];
            packets.push(Self::osc_message(ADDR_OSC_ENABLED, vec![Int(index), Int(patch_osc.is_enabled as i32)]));
            packets.push(Self::osc_message(ADDR_OSC_AMP, vec![Int(index), Float(patch_osc.amplitude as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_FREE_PHASE, vec![Int(index), Int(patch_osc.is_free_phase as i32)]));
            packets.push(Self::osc_message(ADDR_OSC_INITIAL_PHASE, vec![Int(index), Float(patch_osc.initial_phase as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_FIXED_FREQ, vec![Int(index), Int(patch_osc.is_fixed_freq as i32)]));
            packets.push(Self::osc_message(ADDR_OSC_FREQ, vec![Int(index), Float(patch_osc.base_frequency as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_OCTAVES, vec![Int(index), Float(patch_osc.octaves as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_SEMITONES, vec![Int(index), Float(patch_osc.semitones as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_DETUNE, vec![Int(index), Float(patch_osc.detune as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_LEVEL, vec![Int(index), Float(patch_osc.level as f32)]));
            packets.push(Self::osc_message(ADDR_OSC_PAN, vec![Int(index), Float(patch_osc.panning as f32)]));
            for j in 0..MAX_OSCILLATORS {
                let level = match patch_osc.amp_mod.get(&j) {
                    Some(level) => level.clone() as f32,
                    None => 0.0f32
                };
                packets.push(Self::osc_message(ADDR_OSC_AMP_MOD, vec![Int(j as i32), Int(index), Float(level)]));

                let level = match patch_osc.freq_mod.get(&j) {
                    Some(level) => level.clone() as f32,
                    None => 0.0f32
                };
                packets.push(Self::osc_message(ADDR_OSC_FREQ_MOD, vec![Int(j as i32), Int(index), Float(level)]));
            }
        }
        let packet = OscPacket::Bundle(OscBundle {
            timetag: Time(0, 0),
            content: packets.clone()
        });
        self.output_packets.push(packet);
    }

    fn osc_message(addr: &str, args: Vec<OscType>) -> OscPacket {
        OscPacket::Message(OscMessage {
            addr: addr.to_string(),
            args: Some(args)})
    }

    fn control_note(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((key, velocity)) = args_note(args) {
            if velocity > 0.0 {
                self.note_on(key, velocity);
            }
            else {
                self.note_off(key, velocity);
            }
        }
    }

    fn control_osc_amplitude(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_val(args, -100.0, 100.0) {
            self.patch.borrow_mut().oscillators[index].amplitude = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_frequency(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_val(args, 0.0, 22000.0) {
            let mut patch = self.patch.borrow_mut();
            if patch.oscillators[index].is_fixed_freq {
                patch.oscillators[index].base_frequency = value;
                self.patch_version += 1;
            }
        }
    }

    fn control_osc_octaves(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_val(args, -8.0, 8.0) {
            self.patch.borrow_mut().oscillators[index].octaves = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_semitones(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_val(args, -12.0, 12.0) {
            self.patch.borrow_mut().oscillators[index].semitones = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_level(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_val(args, -1.0, 1.0) {
            self.patch.borrow_mut().oscillators[index].level = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_panning(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_val(args, -1.0, 1.0) {
            self.patch.borrow_mut().oscillators[index].panning = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_fm_mod(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((src_index, dst_index, value)) = args_osc_mod_val(args) {
            self.patch.borrow_mut().oscillators[src_index].freq_mod.insert(dst_index, value);
            self.patch_version += 1;
        }
    }

    fn control_osc_enabled(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_toggle(args) {
            self.patch.borrow_mut().oscillators[index].is_enabled = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_fixed_freq(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_toggle(args) {
            self.patch.borrow_mut().oscillators[index].is_fixed_freq = value;
            self.patch_version += 1;
        }
    }

    fn control_osc_free_phase(&mut self, args: &Option<Vec<OscType>>) {
        if let Some((index, value)) = args_osc_toggle(args) {
            self.patch.borrow_mut().oscillators[index].is_free_phase = value;
            self.patch_version += 1;
        }
    }

    // fn foreach_active_voice<F>(&mut self, mut f: F) where F: FnMut(&mut Voice) {
    //     for voice_index in self.active_voices.iter() {
    //         f(&mut self.voices[*voice_index])
    //     }
    // }

    pub fn process(&mut self) -> (f64, f64) {
        let mut left = 0f64;
        let mut right = 0f64;
        for voice_index in self.active_voices.iter() {
            let ref mut voice = self.voices[*voice_index];
            if voice.patch_version() != self.patch_version {
                voice.update_patch(&self.patch.borrow(), self.patch_version);
            }
            let (voice_left, voice_right) = voice.process();
            left += voice_left;
            right += voice_right;
        }

        (left, right)
    }
}

fn args_note(args: &Option<Vec<OscType>>) -> Option<(usize, f64)> {
    match args {
        &Some(ref args) if args.len() == 2 => {
            match (&args[0], &args[1]) {
                (&OscType::Float(ref key), &OscType::Float(ref value)) => {
                    let key = key.clone() as usize;
                    let velocity = value.clone() as f64;
                    if key < MAX_KEYS && velocity >= 0.0 && velocity <= 1.0 {
                        Some((key, velocity))
                    }
                    else { None }
                },
                _ => None
            }
        },
        &Some(ref args) if args.len() == 4 => {
            match (&args[1], &args[2]) {
                (&OscType::Int(ref key), &OscType::Float(ref value)) => {
                    let key = key.clone() as usize;
                    let velocity = value.clone() as f64;
                    if key < MAX_KEYS && velocity >= 0.0 && velocity <= 1.0 {
                        Some((key, velocity))
                    }
                    else { None }
                },
                _ => None
            }
        },
        _ => None
    }
}

fn args_osc_val(args: &Option<Vec<OscType>>, min: f64, max: f64) -> Option<(usize, f64)> {
    match args {
        &Some(ref args) if args.len() == 2 => {
            match (&args[0], &args[1]) {
                (&OscType::Int(ref index), &OscType::Float(ref value)) => {
                    let index = (index - 1) as usize;
                    let value = value.clone() as f64;
                    if index < MAX_OSCILLATORS && value >= min && value <= max {
                        Some((index, value))
                    }
                    else { None }
                },
                _ => None
            }
        },
        _ => None
    }
}

fn args_osc_toggle(args: &Option<Vec<OscType>>) -> Option<(usize, bool)> {
    match args {
        &Some(ref args) if args.len() == 2 => {
            match (&args[0], &args[1]) {
                (&OscType::Int(ref index), &OscType::Int(ref value)) => {
                    let index = (index - 1) as usize;
                    let value = value.clone() as usize;
                    if index < MAX_OSCILLATORS && (value == 1 || value == 0) {
                        Some((index, value == 1))
                    }
                    else { None }
                },
                _ => None
            }
        },
        _ => None
    }
}

fn args_osc_mod_val(args: &Option<Vec<OscType>>) -> Option<(usize, usize, f64)> {
    match args {
        &Some(ref args) if args.len() == 3 => {
            match (&args[0], &args[1], &args[2]) {
                (&OscType::Int(ref dst_index), &OscType::Int(ref src_index), &OscType::Float(ref value)) => {
                    let src_index = (src_index - 1) as usize;
                    let dst_index = (dst_index - 1) as usize;
                    let value = value.clone() as f64;
                    if src_index < MAX_OSCILLATORS && dst_index < MAX_OSCILLATORS && value >= -1.0 && value <= 1.0 {
                        Some((src_index, dst_index, value))
                    }
                    else { None }
                },
                _ => None
            }
        },
        _ => None
    }
}
