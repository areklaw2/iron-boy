use bitfields::bitfield;

#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct OamAttributes {
    #[bits(3)]
    cgb_palette: u8,
    bank: bool,
    dmg_palette: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}

#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct BgMapAttributes {
    #[bits(3)]
    color_palette: u8,
    bank: bool,
    _reserved: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}
