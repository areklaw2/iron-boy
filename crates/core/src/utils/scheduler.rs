use std::{cell::RefCell, collections::BinaryHeap};

use tracing::debug;

use crate::utils::event::{Event, EventType};

pub struct Scheduler {
    timestamp: usize,
    events: BinaryHeap<RefCell<Event>>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            timestamp: 0,
            events: BinaryHeap::new(),
        }
    }

    pub fn peek(&self) -> Option<EventType> {
        self.events.peek().map(|e| e.borrow().event_type())
    }

    pub fn pop(&mut self) -> Option<(EventType, usize)> {
        loop {
            match self.events.peek() {
                Some(event) => {
                    if self.timestamp >= event.borrow().timestamp() {
                        let event = self.events.pop().unwrap();
                        if event.borrow().is_cancelled() {
                            continue;
                        }

                        debug!(
                            "EVENT_EXECUTED: {:?} at timestamp {} (current: {})",
                            event.borrow().event_type(),
                            event.borrow().timestamp(),
                            self.timestamp
                        );
                        return Some((event.borrow().event_type(), event.borrow().timestamp()));
                    } else {
                        return None;
                    }
                }
                None => return None,
            }
        }
    }

    pub fn cancel_events(&mut self, event_type: EventType) {
        for event in &self.events {
            if event.borrow().event_type() == event_type {
                debug!(
                    "EVENT_CANCELLED: {:?} at timestamp {} (current: {})",
                    event.borrow().event_type(),
                    event.borrow().timestamp(),
                    self.timestamp
                );
                event.borrow_mut().cancel();
            }
        }
    }

    pub fn schedule(&mut self, event_type: EventType, delta_time: usize) {
        let timestamp = self.timestamp + delta_time;
        debug!(
            "EVENT_SCHEDULED: {:?} at timestamp {} (current: {})",
            event_type, timestamp, self.timestamp
        );
        self.events.push(RefCell::new(Event::new(event_type, timestamp)));
    }

    pub fn schedule_at_timestamp(&mut self, event_type: EventType, timestamp: usize) {
        debug!(
            "EVENT_SCHEDULED: {:?} at timestamp {} (current: {})",
            event_type, timestamp, self.timestamp
        );
        self.events.push(RefCell::new(Event::new(event_type, timestamp)));
    }

    pub fn cycles_until_next_event(&self) -> usize {
        match self.events.peek() {
            Some(event) => event.borrow().timestamp() - self.timestamp,
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
            Some(event) => event.borrow().timestamp(),
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
