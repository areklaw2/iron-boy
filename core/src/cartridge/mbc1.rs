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
            rom_banks,
            ram_banks,
            has_battery,
        };
        Ok(mbc)
    }
}

impl MemoryBankController for Mbc1 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = match address {
            0x0000..=0x3FFF => match self.banking_mode == 0 {
                true => 0,
                false => self.current_rom_bank & 0xE0,
            },
            _ => self.current_rom_bank,
        };
        let address = bank * 0x4000 | ((address as usize) & 0x3FFF);
        *self.rom.get(address).unwrap_or(&0xFF)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0xF == 0xA;
            }
            0x2000..=0x3FFF => {
                let bank = match (value as usize) & 0x1F {
                    0 => 1,
                    n => n,
                };
                self.current_rom_bank = ((self.current_rom_bank & 0xE0) | bank) % self.rom_banks;
            }
            0x4000..=0x5FFF => {
                if self.rom_banks > 0x20 {
                    let bits = (value as usize & 0x03) % (self.rom_banks >> 5);
                    self.current_rom_bank = self.current_rom_bank & 0x1F | (bits << 5)
                }
                if self.ram_banks > 1 {
                    self.current_ram_bank = (value as usize) & 0x03;
                }
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value & 0x01;
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        let rambank = if self.banking_mode == 1 { self.current_ram_bank } else { 0 };
        self.ram[(rambank * 0x2000) | ((address & 0x1FFF) as usize)]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }
        let rambank = if self.banking_mode == 1 { self.current_ram_bank } else { 0 };
        let address = (rambank * 0x2000) | ((address & 0x1FFF) as usize);
        if address < self.ram.len() {
            self.ram[address] = value;
            self.ram_updated = true;
        }
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
