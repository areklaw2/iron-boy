use std::{thread, time::Duration};

use crate::{
    apu::Apu,
    boot_rom,
    cartridge::Cartridge,
    io::{joypad::Joypad, serial_transfer::SerialTransfer, timer::Timer},
    ppu::Ppu,
};

pub trait Memory {
    fn mem_read(&mut self, address: u16) -> u8;

    fn mem_read_16(&mut self, address: u16) -> u16 {
        let lo = self.mem_read(address) as u16;
        let hi = self.mem_read(address + 1) as u16;
        hi << 8 | lo
    }

    fn mem_write(&mut self, address: u16, data: u8);

    fn mem_write_16(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xFF) as u8;
        self.mem_write(address, lo);
        self.mem_write(address + 1, hi);
    }
}

const WRAM_SIZE: usize = 0x8000;
const HRAM_SIZE: usize = 0x007F;

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    wram_bank: usize,
    interrupt_enable: u8,
    interupt_flag: u8,
    pub joypad: Joypad,
    pub serial_transfer: SerialTransfer,
    pub timer: Timer,
    pub ppu: Ppu,
    pub apu: Apu,
    boot_rom: bool,
}

impl Memory for Bus {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => {
                if self.boot_rom && address < 0x100 {
                    return boot_rom::BYTES[address as usize];
                }
                self.cartridge.mem_read(address)
            }
            0x8000..=0x9FFF => self.ppu.mem_read(address),
            0xA000..=0xBFFF => self.cartridge.mem_read(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF],
            0xFE00..=0xFE9F => self.ppu.mem_read(address),
            // 0xFEA0..=0xFEFF => panic!("Reserved"),
            0xFF00 => self.joypad.mem_read(address),
            0xFF01..=0xFF02 => self.serial_transfer.mem_read(address),
            0xFF04..=0xFF07 => self.timer.mem_read(address),
            0xFF0F => self.interupt_flag | 0b11100000,
            0xFF10..=0xFF26 => self.apu.mem_read(address),
            0xFF30..=0xFF3F => todo!("Wave pattern"),
            0xFF40..=0xFF4B => self.ppu.mem_read(address),
            0xFF50 => todo!("Set to non-zero to disable boot ROM"),
            0xFF51..=0xFF55 => todo!("VRAM DMA"),
            0xFF56 => todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.mem_read(address),
            0xFF70 => todo!("WRAM Bank Select"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F],
            0xFFFF => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.mem_write(address, data),
            0x8000..=0x9FFF => self.ppu.mem_write(address, data),
            0xA000..=0xBFFF => self.cartridge.mem_write(address, data),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF] = data,
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] = data,
            0xFE00..=0xFE9F => self.ppu.mem_write(address, data),
            //0xFEA0..=0xFEFF => panic!("Reserved"),
            0xFF00 => self.joypad.mem_write(address, data),
            0xFF01..=0xFF02 => self.serial_transfer.mem_write(address, data),
            0xFF04..=0xFF07 => self.timer.mem_write(address, data),
            0xFF0F => self.interupt_flag = data,
            0xFF10..=0xFF26 => self.apu.mem_write(address, data),
            0xFF30..=0xFF3F => todo!("Wave pattern"),
            0xFF40..=0xFF45 => self.ppu.mem_write(address, data),
            0xFF46 => self.oam_dma(data),
            0xFF47..=0xFF4B => self.ppu.mem_write(address, data),
            0xFF50 => {
                if self.boot_rom {
                    if data > 0 {
                        self.boot_rom = false;
                    }
                }
            }
            0xFF51..=0xFF55 => todo!("VRAM DMA"),
            0xFF56 => todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.mem_write(address, data),
            0xFF70 => todo!("WRAM Bank Select CBG"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F] = data,
            0xFFFF => self.interrupt_enable = data,
            _ => {}
        }
    }
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mut bus = Bus {
            cartridge,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            wram_bank: 1,
            interrupt_enable: 0,
            interupt_flag: 0,
            joypad: Joypad::new(),
            serial_transfer: SerialTransfer::new(),
            timer: Timer::new(),
            ppu: Ppu::new(),
            apu: Apu::new(),
            boot_rom: true,
        };

        bus.set_hardware_registers();
        bus
    }

    fn set_hardware_registers(&mut self) {
        self.mem_write(0xFF05, 0);
        self.mem_write(0xFF06, 0);
        self.mem_write(0xFF07, 0);
        self.mem_write(0xFF10, 0x80);
        self.mem_write(0xFF11, 0xBF);
        self.mem_write(0xFF12, 0xF3);
        self.mem_write(0xFF14, 0xBF);
        self.mem_write(0xFF16, 0x3F);
        self.mem_write(0xFF17, 0);
        self.mem_write(0xFF19, 0xBF);
        self.mem_write(0xFF1A, 0x7F);
        self.mem_write(0xFF1B, 0xFF);
        self.mem_write(0xFF1C, 0x9F);
        self.mem_write(0xFF1E, 0xFF);
        self.mem_write(0xFF20, 0xFF);
        self.mem_write(0xFF21, 0);
        self.mem_write(0xFF22, 0);
        self.mem_write(0xFF23, 0xBF);
        self.mem_write(0xFF24, 0x77);
        self.mem_write(0xFF25, 0xF3);
        self.mem_write(0xFF26, 0xF1);
        self.mem_write(0xFF40, 0x91);
        self.mem_write(0xFF42, 0);
        self.mem_write(0xFF43, 0);
        self.mem_write(0xFF45, 0);
        self.mem_write(0xFF47, 0xFC);
        self.mem_write(0xFF48, 0xFF);
        self.mem_write(0xFF49, 0xFF);
        self.mem_write(0xFF4A, 0);
        self.mem_write(0xFF4B, 0);
    }

    pub fn machine_cycle(&mut self, ticks: u32) -> u32 {
        self.interupt_flag |= self.joypad.interrupt;
        self.joypad.interrupt = 0;

        self.interupt_flag |= self.serial_transfer.interrupt;
        self.serial_transfer.interrupt = 0;

        self.timer.cycle(ticks);
        self.interupt_flag |= self.timer.interrupt;
        self.timer.interrupt = 0;

        self.ppu.cycle(ticks);
        self.interupt_flag |= self.ppu.interrupt;
        self.ppu.interrupt = 0;

        return ticks;
    }

    fn oam_dma(&mut self, data: u8) {
        let base = (data as u16) << 8;
        for i in 0..0xA0 {
            let byte = self.mem_read(base + i);
            self.mem_write(0xFE00 + i, byte);
            thread::sleep(Duration::from_secs(2));
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_mem_read_write() {
        // let mut bus = Bus::new(Cartridge::default());
        // bus.mem_write(0x01, 0x55);
        // assert_eq!(bus.mem_read(0x01), 0x55);
    }
}
