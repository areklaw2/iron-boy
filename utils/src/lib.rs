#[derive(PartialEq, Copy, Clone)]
pub enum GbMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}

pub enum Speed {
    Single,
    Double,
}
