use super::MemoryBankController;

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    ram_updated: bool,
    banking_mode: u8,
    current_rom_bank: usize,
    current_ram_bank: usize,
    rom_banks: usize,
    ram_banks: usize,
    has_battery: bool,
}

impl MemoryBankController for Mbc1 {
    fn rom_read(&self, address: u16) -> u8 {
        todo!()
    }

    fn rom_write(&mut self, address: u16, data: u8) {
        todo!()
    }

    fn ram_read(&self, address: u16) -> u8 {
        todo!()
    }

    fn ram_write(&mut self, address: u16, data: u8) {
        todo!()
    }

    fn ram_updated(&mut self) -> bool {
        let result = self.ram_updated;
        self.ram_updated = false;
        result
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }

    fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() != self.ram.len() {
            return Err("Data with incorrect length being loaded");
        }

        self.ram = data.to_vec();
        Ok(())
    }

    fn dump_ram(&self) -> Vec<u8> {
        self.ram.to_vec()
    }
}

impl Mbc1 {
    pub fn new(buffer: Vec<u8>, rom_banks: usize, ram_banks: usize, has_battery: bool) -> Result<Mbc1, &'static str> {
        let mbc = Mbc1 {
            rom: buffer,
            ram: vec![0; ram_banks * 0x2000],
            ram_enabled: false,
            ram_updated: false,
            banking_mode: 0,
            current_rom_bank: 1,
            current_ram_bank: 0,
            rom_banks: rom_banks,
            ram_banks: ram_banks,
            has_battery: has_battery,
        };
        Ok(mbc)
    }
}
