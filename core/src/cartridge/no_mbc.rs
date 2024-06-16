use super::MemoryBankController;

pub struct NoMbc {
    rom: Vec<u8>,
}

impl NoMbc {
    pub fn new(buffer: Vec<u8>) -> Result<NoMbc, &'static str> {
        Ok(NoMbc { rom: buffer })
    }
}

impl MemoryBankController for NoMbc {
    fn rom_read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn rom_write(&mut self, _address: u16, _data: u8) {
        return;
    }

    fn ram_read(&self, _address: u16) -> u8 {
        0
    }

    fn ram_write(&mut self, _address: u16, _data: u8) {
        return;
    }

    fn check_and_reset_ram_updated(&mut self) -> bool {
        false
    }

    fn has_battery(&self) -> bool {
        false
    }

    fn load_ram(&mut self, _data: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }

    fn dump_ram(&self) -> Vec<u8> {
        Vec::new()
    }
}
