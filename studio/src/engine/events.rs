use std::collections::btree_map::{self, BTreeMap, Entry};

use rosc::OscPacket;

use engine::types::Timestamp;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    NoteOn { key: u8, velocity: f64 },
    NoteOff { key: u8, velocity: f64 },
    Control(OscPacket),
}

#[derive(Debug, Clone)]
pub struct Event {
    timestamp: Timestamp,
    message: Message
}

impl Event {
    pub fn new(timestamp: Timestamp, message: Message) -> Event {
        Event {
            timestamp: timestamp,
            message: message
        }
    }

    #[inline]
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    #[inline]
    pub fn message(&self) -> &Message {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub enum Port {
    Midi(String),
    Osc(String),
}

#[derive(Debug, Clone)]
pub struct PortEvents {
    port: Port,
    events: Vec<Event>
}

impl PortEvents {
    pub fn new(port: Port, events: Vec<Event>) -> Self {
        PortEvents {
            port: port,
            events: events
        }
    }

    #[inline]
    pub fn port(&self) -> &Port {
        &self.port
    }

    #[inline]
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}

pub struct EventsBuffer(BTreeMap<Timestamp, Vec<Message>>);

impl EventsBuffer {
    pub fn new() -> Self {
        EventsBuffer(BTreeMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, event: &Event) {
        let message = event.message().clone();
        match self.0.entry(event.timestamp()) {
            Entry::Occupied(ref mut entry) => entry.get_mut().push(message),
            Entry::Vacant(entry) => { entry.insert(vec![message]); }
        }
    }

    pub fn split(&mut self, until: Timestamp) -> EventsBuffer {
        let mut map = self.0.to_owned();
        self.0 = map.split_off(&until);
        EventsBuffer(map)
    }

    pub fn iter(&self) -> Iter {
        Iter(self.0.iter())
    }

    // pub fn pop_until(&mut self, until: Timestamp) -> Vec<Event> {
    //     let events = Vec::new();
    //     let iter = self.0.iter();
    //     while let Some((timestamp, messages)) = iter.next() {
    //         events.extend(messages.iter().map(|msg| Event::new(timestamp, msg.clone())));
    //     }
    //     events
    // }
}

pub struct Iter<'a>(btree_map::Iter<'a, Timestamp, Vec<Message>>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Timestamp, &'a Vec<Message>);

    fn next(&mut self) -> Option<(&'a Timestamp, &'a Vec<Message>)> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_buffer_len() {
        let mut eb = EventsBuffer::new();
        let evt = Event::new(1, Message::NoteOn {key: 0, velocity: 0.1});
        assert_eq!(eb.len(), 0);
        assert_eq!(eb.is_empty(), true);
        eb.push(&evt);
        assert_eq!(eb.len(), 1);
        assert_eq!(eb.is_empty(), false);
    }

    #[test]
    fn events_buffer_push() {
        let mut eb = EventsBuffer::new();
        let msg1 = Message::NoteOn {key: 10, velocity: 0.1};
        let msg2 = Message::NoteOn {key: 20, velocity: 0.1};
        let msg3 = Message::NoteOn {key: 30, velocity: 0.1};
        eb.push(&Event::new(1, msg1.clone()));
        eb.push(&Event::new(2, msg2.clone()));
        eb.push(&Event::new(1, msg3.clone()));
        assert_eq!(eb.0.get(&1), Some(&vec![msg1, msg3]));
        assert_eq!(eb.0.get(&2), Some(&vec![msg2]));
    }

    #[test]
    fn events_buffer_split() {
        let mut eb = EventsBuffer::new();
        let msg1 = Message::NoteOn {key: 10, velocity: 0.1};
        let msg2 = Message::NoteOn {key: 20, velocity: 0.1};
        let msg3 = Message::NoteOn {key: 30, velocity: 0.1};
        eb.push(&Event::new(1, msg1.clone()));
        eb.push(&Event::new(3, msg2.clone()));
        eb.push(&Event::new(1, msg3.clone()));
        let low = eb.split(2);
        assert_eq!(low.0.get(&1), Some(&vec![msg1, msg3]));
        assert_eq!(eb.0.get(&3), Some(&vec![msg2]));
    }

    #[test]
    fn events_buffer_iter() {
        let mut eb = EventsBuffer::new();
        let msg1 = Message::NoteOn {key: 10, velocity: 0.1};
        let msg2 = Message::NoteOn {key: 20, velocity: 0.1};
        let msg3 = Message::NoteOn {key: 30, velocity: 0.1};
        eb.push(&Event::new(1, msg1.clone()));
        eb.push(&Event::new(3, msg2.clone()));
        eb.push(&Event::new(1, msg3.clone()));
        let mut it = eb.iter();
        assert_eq!(it.next(), Some((&1, &vec![msg1, msg3])));
        assert_eq!(it.next(), Some((&3, &vec![msg2])));
    }
}
