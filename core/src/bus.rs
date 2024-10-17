use crate::{
    apu::Apu,
    boot_rom,
    cartridge::Cartridge,
    io::{joypad::JoyPad, serial_transfer::SerialTransfer, timer::Timer},
    ppu::Ppu,
};

pub trait MemoryAccess {
    fn read_8(&self, address: u16) -> u8;

    fn read_16(&self, address: u16) -> u16 {
        let lo = self.read_8(address) as u16;
        let hi = self.read_8(address + 1) as u16;
        hi << 8 | lo
    }

    fn write_8(&mut self, address: u16, value: u8);

    fn write_16(&mut self, address: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.write_8(address, lo);
        self.write_8(address + 1, hi);
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
    interrupt_flag: u8,
    pub joy_pad: JoyPad,
    pub serial_transfer: SerialTransfer,
    pub timer: Timer,
    pub ppu: Ppu,
    pub apu: Apu,
    boot_rom: bool,
}

impl MemoryAccess for Bus {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => {
                // figure out how to make this toggleable
                if self.boot_rom && address < 0x100 {
                    return boot_rom::BYTES[address as usize];
                }
                self.cartridge.mbc.read_rom(address)
            }
            0x8000..=0x9FFF => self.ppu.read_8(address),
            0xA000..=0xBFFF => self.cartridge.mbc.read_ram(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF],
            0xFE00..=0xFE9F => self.ppu.read_8(address),
            0xFF00 => self.joy_pad.read_8(address),
            0xFF01..=0xFF02 => self.serial_transfer.read_8(address),
            0xFF04..=0xFF07 => self.timer.read_8(address),
            0xFF0F => self.interrupt_flag | 0b11100000,
            0xFF10..=0xFF3F => self.apu.read_8(address),
            0xFF40..=0xFF4B => self.ppu.read_8(address),
            0xFF50 => todo!("Set to non-zero to disable boot ROM"),
            0xFF51..=0xFF55 => todo!("VRAM DMA"),
            0xFF56 => todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.read_8(address),
            0xFF70 => todo!("WRAM Bank Select"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F],
            0xFFFF => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.mbc.write_rom(address, value),
            0x8000..=0x9FFF => self.ppu.write_8(address, value),
            0xA000..=0xBFFF => self.cartridge.mbc.write_ram(address, value),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF] = value,
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] = value,
            0xFE00..=0xFE9F => self.ppu.write_8(address, value),
            0xFF00 => self.joy_pad.write_8(address, value),
            0xFF01..=0xFF02 => self.serial_transfer.write_8(address, value),
            0xFF04..=0xFF07 => self.timer.write_8(address, value),
            0xFF0F => self.interrupt_flag = value,
            0xFF10..=0xFF3F => self.apu.write_8(address, value),
            0xFF40..=0xFF45 => self.ppu.write_8(address, value),
            0xFF46 => self.oam_dma(value),
            0xFF47..=0xFF4B => self.ppu.write_8(address, value),
            0xFF50 => {
                if self.boot_rom {
                    if value > 0 {
                        self.boot_rom = false;
                    }
                }
            }
            0xFF51..=0xFF55 => todo!("VRAM DMA"),
            0xFF56 => todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.write_8(address, value),
            0xFF70 => todo!("WRAM Bank Select CBG"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F] = value,
            0xFFFF => self.interrupt_enable = value,
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
            interrupt_flag: 0,
            joy_pad: JoyPad::new(),
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
        self.write_8(0xFF05, 0);
        self.write_8(0xFF06, 0);
        self.write_8(0xFF07, 0);
        self.write_8(0xFF10, 0x80);
        self.write_8(0xFF11, 0xBF);
        self.write_8(0xFF12, 0xF3);
        self.write_8(0xFF14, 0xBF);
        self.write_8(0xFF16, 0x3F);
        self.write_8(0xFF17, 0);
        self.write_8(0xFF19, 0xBF);
        self.write_8(0xFF1A, 0x7F);
        self.write_8(0xFF1B, 0xFF);
        self.write_8(0xFF1C, 0x9F);
        self.write_8(0xFF1E, 0xFF);
        self.write_8(0xFF20, 0xFF);
        self.write_8(0xFF21, 0);
        self.write_8(0xFF22, 0);
        self.write_8(0xFF23, 0xBF);
        self.write_8(0xFF24, 0x77);
        self.write_8(0xFF25, 0xF3);
        self.write_8(0xFF26, 0xF1);
        self.write_8(0xFF40, 0x91);
        self.write_8(0xFF42, 0);
        self.write_8(0xFF43, 0);
        self.write_8(0xFF45, 0);
        self.write_8(0xFF47, 0xFC);
        self.write_8(0xFF48, 0xFF);
        self.write_8(0xFF49, 0xFF);
        self.write_8(0xFF4A, 0);
        self.write_8(0xFF4B, 0);
    }

    pub fn machine_cycle(&mut self, ticks: u32) -> u32 {
        self.interrupt_flag |= self.joy_pad.interrupt;
        self.joy_pad.interrupt = 0;

        self.interrupt_flag |= self.serial_transfer.interrupt;
        self.serial_transfer.interrupt = 0;

        self.timer.cycle(ticks);
        self.interrupt_flag |= self.timer.interrupt;
        self.timer.interrupt = 0;

        self.ppu.cycle(ticks);
        self.interrupt_flag |= self.ppu.interrupt;
        self.ppu.interrupt = 0;

        self.apu.cycle(ticks);

        return ticks;
    }

    fn oam_dma(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xA0 {
            let byte = self.read_8(base + i);
            self.write_8(0xFE00 + i, byte);
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
