use crate::ppu::attributes::OamAttributes;

#[derive(Debug, Copy, Clone)]
pub struct Oam {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    attributes: OamAttributes,
}

impl Oam {
    pub fn new() -> Self {
        Self {
            y_position: 0,
            x_position: 0,
            tile_index: 0,
            attributes: OamAttributes::from_bits(0),
        }
    }

    pub fn y_position(&self) -> u8 {
        self.y_position
    }
    pub fn set_y_position(&mut self, value: u8) {
        self.y_position = value;
    }

    pub fn x_position(&self) -> u8 {
        self.x_position
    }
    pub fn set_x_position(&mut self, value: u8) {
        self.x_position = value;
    }

    pub fn tile_index(&self) -> u8 {
        self.tile_index
    }
    pub fn set_tile_index(&mut self, value: u8) {
        self.tile_index = value;
    }

    pub fn attributes(&self) -> &OamAttributes {
        &self.attributes
    }
    pub fn set_attributes(&mut self, value: u8) {
        self.attributes.set_bits(value)
    }
}
