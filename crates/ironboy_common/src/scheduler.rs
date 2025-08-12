use std::collections::BinaryHeap;

use crate::event::{Event, EventType};

pub struct Scheduler {
    timestamp: usize,
    events: BinaryHeap<Event>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            timestamp: 0,
            events: BinaryHeap::new(),
        }
    }

    pub fn peek(&self) -> Option<EventType> {
        self.events.peek().map(|e| e.event_type())
    }

    pub fn pop(&mut self) -> Option<(EventType, usize)> {
        match self.events.peek() {
            Some(event) => {
                if self.timestamp >= event.timestamp() {
                    let event = self.events.pop().unwrap_or_else(|| unreachable!());
                    Some((event.event_type(), event.timestamp()))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn cancel_events(&mut self, event_type: EventType) {
        let mut new_events = BinaryHeap::new();
        self.events
            .iter()
            .filter(|e| e.event_type() != event_type)
            .for_each(|e| new_events.push(e.clone()));
        self.events = new_events
    }

    pub fn schedule(&mut self, event_type: EventType, delta_time: usize) {
        self.events.push(Event::new(event_type, self.timestamp + delta_time));
    }

    pub fn schedule_at_timestamp(&mut self, event_type: EventType, timestamp: usize) {
        self.events.push(Event::new(event_type, timestamp));
    }

    pub fn cycles_until_next_event(&self) -> usize {
        match self.events.peek() {
            Some(event) => event.timestamp() - self.timestamp,
            None => 0,
        }
    }

    pub fn update(&mut self, cycles: usize) {
        self.timestamp += cycles;
    }

    pub fn update_to_next_event(&mut self) {
        self.timestamp += self.cycles_until_next_event();
    }

    pub fn timestamp_of_next_event(&self) -> usize {
        match self.events.peek() {
            Some(event) => event.timestamp(),
            None => panic!("No events"),
        }
    }

    pub fn timestamp(&self) -> usize {
        self.timestamp
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
