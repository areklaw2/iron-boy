use std::convert::TryInto;
use std::io::prelude::*;

use super::rtc::RealTimeClock;
use super::MemoryBankController;

pub struct Mbc3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_updated: bool,
    ram_enabled: bool,
    current_rom_bank: usize,
    current_ram_bank: usize,
    ram_banks: usize,
    has_battery: bool,
    select_rtc_register: bool,
    rtc: RealTimeClock,
}

impl Mbc3 {
    pub fn new(buffer: Vec<u8>, ram_banks: usize, has_ram: bool, has_battery: bool, has_real_time_clock: bool) -> Result<Mbc3, &'static str> {
        let ram_banks = match has_ram {
            true => ram_banks,
            false => 0,
        };

        let mbc = Mbc3 {
            rom: buffer,
            ram: vec![0; ram_banks * 0x2000],
            ram_enabled: false,
            ram_updated: false,
            current_rom_bank: 1,
            current_ram_bank: 0,
            ram_banks,
            has_battery,
            select_rtc_register: false,
            rtc: RealTimeClock::new(has_real_time_clock),
        };
        Ok(mbc)
    }
}

impl MemoryBankController for Mbc3 {
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
            0x0000..=0x1FFF => self.ram_enabled = (value & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                self.current_rom_bank = match value & 0x7F {
                    0 => 1,
                    n => n as usize,
                }
            }
            0x4000..=0x5FFF => {
                self.select_rtc_register = value & 0x8 == 0x8;
                self.current_ram_bank = (value & 0x7) as usize;
            }
            0x6000..=0x7FFF => self.rtc.set_latch_registers(),
            _ => panic!("Could not write to {:04X} (MBC3)", address),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        if !self.select_rtc_register && self.current_ram_bank < self.ram_banks {
            self.ram[self.current_ram_bank * 0x2000 | ((address as usize) & 0x1FFF)]
        } else if self.select_rtc_register && self.current_ram_bank < 5 {
            self.rtc.latch_register(self.current_ram_bank)
        } else {
            0xFF
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }
        if !self.select_rtc_register && self.current_ram_bank < self.ram_banks {
            self.ram[self.current_ram_bank * 0x2000 | ((address as usize) & 0x1FFF)] = value;
            self.ram_updated = true;
        } else if self.select_rtc_register && self.current_ram_bank < 5 {
            self.rtc.set_registers();
            let register_mask = match self.current_ram_bank {
                0 | 1 => 0x3F,
                2 => 0x1F,
                4 => 0xC1,
                _ => 0xFF,
            };
            self.rtc.set_register(self.current_ram_bank, value & register_mask);
            self.rtc.set_time();
            self.ram_updated = true;
        }
    }

    fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if data.len() != self.ram.len() {
            return Err("Data with incorrect length being loaded");
        }

        let (int_bytes, rest) = data.split_at(8);
        let time = u64::from_be_bytes(int_bytes.try_into().unwrap());
        if self.rtc.time().is_some() {
            self.rtc.load_time(Some(time));
        }
        self.ram = rest.to_vec();
        Ok(())
    }

    fn dump_ram(&self) -> Vec<u8> {
        let time = match self.rtc.time() {
            Some(t) => t,
            None => 0,
        };

        let mut file = vec![];
        let mut success = true;
        if success {
            let rtc_bytes = time.to_be_bytes();
            success = file.write_all(&rtc_bytes).is_ok();
        };
        if success {
            let _ = file.write_all(&*self.ram);
        };

        file
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
