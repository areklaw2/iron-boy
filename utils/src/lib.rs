#[derive(PartialEq, Copy, Clone)]
pub enum GameBoyMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}

pub enum Speed {
    Single,
    Double,
}
