use crate::{bus::Bus, register::Registors};

pub struct Cpu {
    registers: Registors,
    bus: Bus,
}
