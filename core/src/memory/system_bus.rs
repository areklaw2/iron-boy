use crate::{
    apu::Apu,
    cartridge::Cartridge,
    io::{joypad::JoyPad, serial_transfer::SerialTransfer, timer::Timer},
    ppu::Ppu,
    GameBoyMode,
};

use super::{IoMemoryAccess, MemoryInterface};

const WRAM_SIZE: usize = 0x8000;
const HRAM_SIZE: usize = 0x007F;

#[derive(Debug, PartialEq)]
enum TransferMode {
    Stopped,
    GeneralPurpose,
    HBlank,
}

pub struct SystemBus {
    cartridge: Cartridge,
    game_boy_mode: GameBoyMode,
    double_speed: bool,
    speed_switch_armed: bool,
    wram_bank: usize,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    hdma_mode: TransferMode,
    hdma_source: u16,
    hdma_destination: u16,
    hdma_length: u8,
    interrupt_enable: u8,
    interrupt_flag: u8,
    undocumented_cgb_registers: [u8; 3],
    pub joy_pad: JoyPad,
    pub serial_transfer: SerialTransfer,
    pub timer: Timer,
    pub ppu: Ppu,
    pub apu: Apu,
}

impl IoMemoryAccess for SystemBus {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.mbc.read_rom(address),
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
            0xFF4D | 0xFF4F | 0xFF51..=0xFF56 | 0xFF70 | 0xFF72..=0xFF77 if self.game_boy_mode != GameBoyMode::Color => 0xFF,
            0xFF4D => ((self.double_speed as u8) << 7) | 0x7E | (self.speed_switch_armed as u8),
            0xFF4F => self.ppu.read_8(address),
            0xFF50 => todo!("Set to non-zero to disable boot ROM"),
            0xFF51..=0xFF55 => self.read_hdma(address),
            0xFF56 => 0xFF, //todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.read_8(address),
            0xFF70 => self.wram_bank as u8,
            0xFF72..=0xFF73 => self.undocumented_cgb_registers[address as usize - 0xFF72],
            0xFF75 => self.undocumented_cgb_registers[2] | 0x8F,
            0xFF76..=0xFF77 => 0x00, //todo!("PCM")
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
            0xFF4D | 0xFF4F | 0xFF51..=0xFF56 | 0xFF70 | 0xFF72..=0xFF77 if self.game_boy_mode != GameBoyMode::Color => {}
            0xFF4D => self.speed_switch_armed = value & 0x1 != 0,
            0xFF4F => self.ppu.write_8(address, value),
            0xFF50 => {}
            0xFF51..=0xFF55 => self.write_hdma(address, value),
            0xFF56 => {} //todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.write_8(address, value),
            0xFF70 => {
                self.wram_bank = match value & 0x7 {
                    0 => 1,
                    n => n as usize,
                };
            }
            0xFF72..=0xFF73 => self.undocumented_cgb_registers[address as usize - 0xFF72] = value,
            0xFF75 => self.undocumented_cgb_registers[2] = value,
            0xFF76..=0xFF77 => {} //todo!("PCM"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F] = value,
            0xFFFF => self.interrupt_enable = value,
            _ => {}
        }
    }
}

impl MemoryInterface for SystemBus {
    fn load_8(&self, address: u16) -> u8 {
        self.read_8(address)
    }

    fn store_8(&mut self, address: u16, value: u8) {
        self.write_8(address, value);
    }

    fn cycle(&mut self, cycles: u32, cpu_halted: bool) -> u32 {
        let speed = if self.double_speed { 2 } else { 1 };
        let vram_cycles = self.vram_dma_cycle(cpu_halted);
        let cpu_cycles = cycles + vram_cycles * speed;
        let ppu_cycles = cycles / speed + vram_cycles;

        self.timer.cycle(cpu_cycles);
        self.interrupt_flag |= self.timer.interrupt;
        self.timer.interrupt = 0;

        self.ppu.cycle(ppu_cycles);
        self.interrupt_flag |= self.ppu.interrupt;
        self.ppu.interrupt = 0;

        self.apu.cycle(ppu_cycles);

        self.interrupt_flag |= self.joy_pad.interrupt;
        self.joy_pad.interrupt = 0;

        self.interrupt_flag |= self.serial_transfer.interrupt;
        self.serial_transfer.interrupt = 0;

        cycles
    }

    fn change_speed(&mut self) {
        if self.speed_switch_armed {
            self.double_speed = !self.double_speed;
        }
        self.speed_switch_armed = false;
    }
}

impl SystemBus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mode = cartridge.mode();
        let mut bus = SystemBus {
            cartridge,
            game_boy_mode: mode,
            double_speed: false,
            speed_switch_armed: false,
            wram_bank: 1,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            hdma_source: 0,
            hdma_destination: 0,
            hdma_mode: TransferMode::Stopped,
            hdma_length: 0xFF,
            interrupt_enable: 0,
            interrupt_flag: 0,
            undocumented_cgb_registers: [0; 3],
            joy_pad: JoyPad::new(),
            serial_transfer: SerialTransfer::new(),
            timer: Timer::new(),
            ppu: Ppu::new(mode),
            apu: Apu::new(),
        };

        bus.set_hardware_registers();
        bus
    }

    fn set_hardware_registers(&mut self) {
        self.store_8(0xFF05, 0);
        self.store_8(0xFF06, 0);
        self.store_8(0xFF07, 0);
        self.store_8(0xFF10, 0x80);
        self.store_8(0xFF11, 0xBF);
        self.store_8(0xFF12, 0xF3);
        self.store_8(0xFF14, 0xBF);
        self.store_8(0xFF16, 0x3F);
        self.store_8(0xFF17, 0);
        self.store_8(0xFF19, 0xBF);
        self.store_8(0xFF1A, 0x7F);
        self.store_8(0xFF1B, 0xFF);
        self.store_8(0xFF1C, 0x9F);
        self.store_8(0xFF1E, 0xFF);
        self.store_8(0xFF20, 0xFF);
        self.store_8(0xFF21, 0);
        self.store_8(0xFF22, 0);
        self.store_8(0xFF23, 0xBF);
        self.store_8(0xFF24, 0x77);
        self.store_8(0xFF25, 0xF3);
        self.store_8(0xFF26, 0xF1);
        self.store_8(0xFF40, 0x91);
        self.store_8(0xFF42, 0);
        self.store_8(0xFF43, 0);
        self.store_8(0xFF45, 0);
        self.store_8(0xFF47, 0xFC);
        self.store_8(0xFF48, 0xFF);
        self.store_8(0xFF49, 0xFF);
        self.store_8(0xFF4A, 0);
        self.store_8(0xFF4B, 0);
    }

    pub fn oam_dma(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xA0 {
            let byte = self.load_8(base + i);
            self.store_8(0xFE00 + i, byte);
        }
    }

    fn read_hdma(&self, address: u16) -> u8 {
        match address {
            0xFF51..=0xFF54 => 0xFF,
            0xFF55 => ((self.hdma_mode == TransferMode::Stopped) as u8) << 7 | self.hdma_length,
            _ => panic!("HDMA does not handle read {:04X}", address),
        }
    }

    fn write_hdma(&mut self, address: u16, value: u8) {
        match address {
            0xFF51 => self.hdma_source = (self.hdma_source & 0x00FF) | (value as u16) << 8,
            0xFF52 => self.hdma_source = (self.hdma_source & 0xFF00) | (value & 0xF0) as u16,
            0xFF53 => self.hdma_destination = (self.hdma_destination & 0x00FF) | ((value & 0x1F) as u16) << 8,
            0xFF54 => self.hdma_destination = (self.hdma_destination & 0xFF00) | (value & 0xF0) as u16,
            0xFF55 => match self.hdma_mode {
                TransferMode::HBlank => {
                    if value & 0x80 == 0 {
                        self.hdma_mode = TransferMode::Stopped;
                    }
                }
                TransferMode::Stopped => {
                    self.hdma_mode = match value & 0x80 != 0 {
                        true => TransferMode::HBlank,
                        false => TransferMode::GeneralPurpose,
                    };
                    self.hdma_length = (value & 0x7F) + 1;
                }
                TransferMode::GeneralPurpose => panic!("Cannot cancel General Purpose DMA"),
            },
            _ => panic!("HDMA does not handle write {:04X}", address),
        };
    }

    fn vram_dma_cycle(&mut self, cpu_halted: bool) -> u32 {
        match self.hdma_mode {
            TransferMode::Stopped => 0,
            TransferMode::GeneralPurpose => self.general_purpose_dma(),
            TransferMode::HBlank => self.hblank_dma(cpu_halted),
        }
    }

    fn general_purpose_dma(&mut self) -> u32 {
        let length = self.hdma_length as u32;
        for _ in 0..length {
            for _ in 0..0x10 {
                let b: u8 = self.read_8(self.hdma_source);
                self.ppu.write_8(self.hdma_destination | 0x8000, b);
                self.hdma_source += 1;
                self.hdma_destination += 1;
            }

            if self.hdma_length != 0 {
                self.hdma_length -= 1;
            }
        }

        self.hdma_mode = TransferMode::Stopped;
        length * 8
    }

    fn hblank_dma(&mut self, halted: bool) -> u32 {
        if !self.ppu.is_hblanking() || halted {
            return 0;
        }

        for _ in 0..0x10 {
            let b: u8 = self.read_8(self.hdma_source);
            self.ppu.write_8(self.hdma_destination | 0x8000, b);
            self.hdma_source += 1;
            self.hdma_destination += 1;
        }

        if self.hdma_length != 0 {
            self.hdma_length -= 1;
        }

        if self.hdma_length == 0 {
            self.hdma_mode = TransferMode::Stopped;
        }

        8
    }
}
