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
pub struct DeviceEvents {
    device: String,
    events: Vec<Event>
}

impl DeviceEvents {
    pub fn new(device: &str, events: Vec<Event>) -> DeviceEvents {
        DeviceEvents {
            device: device.to_string(),
            events: events
        }
    }

    #[inline]
    pub fn device(&self) -> &str {
        &self.device
    }

    #[inline]
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}
