// use std::slice::Iter;

use midi::messages::Message;
use midi::types::Timestamp;

#[derive(Clone, Eq, PartialEq)]
pub struct Event {
    timestamp: Timestamp,
    message: Message
}

impl Event {
    pub fn new(timestamp: Timestamp, message: Message) -> Event {
        Event { timestamp: timestamp, message: message }
    }

    #[inline]
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    #[inline]
    pub fn message(&self) -> Message {
        self.message.clone()
    }
}

#[derive(Clone)]
pub struct PortEvents {
    port: String,
    events: Vec<Event>
}

impl PortEvents {
    pub fn new(port: &str, events: Vec<Event>) -> PortEvents {
        PortEvents {
            port: port.to_string(),
            events: events
        }
    }

    #[inline]
    pub fn port(&self) -> &str {
        &self.port
    }

    #[inline]
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}
