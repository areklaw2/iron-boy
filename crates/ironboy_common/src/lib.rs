pub mod constants;
pub mod event;
pub mod memory;
pub mod scheduler;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameBoyMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}
