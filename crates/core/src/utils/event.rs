use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PpuEvent {
    HBlank,
    VBlank,
    OamScan,
    DrawingPixels,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ApuEvent {
    // Frame Sequencing
    LengthTimer,
    Sweep,
    VolumeEnvelope,
    // Channels
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    //Output
    Sample,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TimerEvent {
    DivOverflow,
    TimaOverflow,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum EventType {
    FrameComplete,
    Timer(TimerEvent),
    Ppu(PpuEvent),
    Apu(ApuEvent),
}

#[derive(Debug, Clone, Eq)]
pub struct Event {
    event_type: EventType,
    timestamp: usize,
    cancelled: bool,
}

impl Event {
    pub fn new(event_type: EventType, timestamp: usize) -> Event {
        Event {
            event_type,
            timestamp,
            cancelled: false,
        }
    }

    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    pub fn timestamp(&self) -> usize {
        self.timestamp
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.timestamp.partial_cmp(&self.timestamp)
    }

    fn lt(&self, other: &Self) -> bool {
        other.timestamp < self.timestamp
    }

    fn le(&self, other: &Self) -> bool {
        other.timestamp <= self.timestamp
    }

    fn gt(&self, other: &Self) -> bool {
        other.timestamp > self.timestamp
    }

    fn ge(&self, other: &Self) -> bool {
        other.timestamp >= self.timestamp
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}
