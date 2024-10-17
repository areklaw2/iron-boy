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
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_rom(&mut self, _address: u16, _value: u8) {
        return;
    }

    fn read_ram(&self, _address: u16) -> u8 {
        0
    }

    fn write_ram(&mut self, _address: u16, _value: u8) {
        return;
    }

    fn load_ram(&mut self, _data: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }

    fn dump_ram(&self) -> Vec<u8> {
        Vec::new()
    }

    fn ram_updated(&mut self) -> bool {
        false
    }

    fn has_battery(&self) -> bool {
        false
    }
}
