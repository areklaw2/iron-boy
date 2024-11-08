use super::MemoryBankController;

pub struct Mbc5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    ram_updated: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
    rom_banks: usize,
    ram_banks: usize,
    has_battery: bool,
}

impl Mbc5 {
    pub fn new(buffer: Vec<u8>, rom_banks: usize, ram_banks: usize, has_battery: bool) -> Result<Mbc5, &'static str> {
        let mbc = Mbc5 {
            rom: buffer,
            ram: vec![0; ram_banks * 0x2000],
            ram_enabled: false,
            ram_updated: false,
            current_rom_bank: 1,
            current_ram_bank: 0,
            rom_banks,
            ram_banks,
            has_battery,
        };
        Ok(mbc)
    }
}

impl MemoryBankController for Mbc5 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = match address {
            0x0000..=0x3FFF => 0,
            _ => self.current_rom_bank,
        };
        let address = bank * 0x4000 | ((address as usize) & 0x3FFF);
        *self.rom.get(address).unwrap_or(&0xFF)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0x0F == 0x0A,
            0x2000..=0x2FFF => self.current_rom_bank = ((self.current_rom_bank & 0x100) | (value as usize)) % self.rom_banks,
            0x3000..=0x3FFF => self.current_rom_bank = ((self.current_rom_bank & 0x0FF) | (((value & 0x1) as usize) << 8)) % self.rom_banks,
            0x4000..=0x5FFF => self.current_ram_bank = ((value & 0x0F) as usize) % self.ram_banks,
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }
        self.ram[self.current_ram_bank * 0x2000 | ((address as usize) & 0x1FFF)]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.ram_enabled == false {
            return;
        }
        self.ram[self.current_ram_bank * 0x2000 | ((address as usize) & 0x1FFF)] = value;
        self.ram_updated = true;
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

    fn ram_updated(&mut self) -> bool {
        let result = self.ram_updated;
        self.ram_updated = false;
        result
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }
}
