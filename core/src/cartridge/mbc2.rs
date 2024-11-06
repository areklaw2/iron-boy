use super::MemoryBankController;

pub struct Mbc2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_enabled: bool,
    ram_updated: bool,
    current_rom_bank: usize,
    rom_banks: usize,
    has_battery: bool,
}

impl Mbc2 {
    pub fn new(buffer: Vec<u8>, rom_banks: usize, has_battery: bool) -> Result<Mbc2, &'static str> {
        let mbc = Mbc2 {
            rom: buffer,
            ram: vec![0; 512],
            ram_enabled: false,
            ram_updated: false,
            current_rom_bank: 1,
            rom_banks,
            has_battery,
        };
        Ok(mbc)
    }
}

impl MemoryBankController for Mbc2 {
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
            0x0000..=0x3FFF => {
                if address & 0x100 == 0 {
                    self.ram_enabled = value & 0xF == 0xA;
                } else {
                    self.current_rom_bank = match (value as usize) & 0x0F {
                        0 => 1,
                        n => n,
                    } % self.rom_banks;
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        self.ram[(address as usize) & 0x1FF] | 0xF0
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }
        self.ram[(address as usize) & 0x1FF] = value | 0xF0;
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
