pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

pub struct Interrupts {
    interrupt_master_enable: bool,
    enable_interrupt: u8,
    disable_interrupt: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            interrupt_master_enable: false,
            enable_interrupt: 0,
            disable_interrupt: 0,
        }
    }

    pub fn ime(&self) -> bool {
        self.interrupt_master_enable
    }

    pub fn set_ime(&mut self, value: bool) {
        self.interrupt_master_enable = value;
    }

    pub fn update_ime(&mut self) {
        if self.disable_interrupt == 1 {
            self.interrupt_master_enable = false;
        }
        self.disable_interrupt = self.disable_interrupt.saturating_sub(1);

        if self.enable_interrupt == 1 {
            self.interrupt_master_enable = true;
        }
        self.enable_interrupt = self.enable_interrupt.saturating_sub(1);
    }

    pub fn set_ei(&mut self) {
        self.enable_interrupt = 2
    }

    pub fn set_di(&mut self) {
        self.disable_interrupt = 2
    }
}
