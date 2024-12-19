use crate::{
    apu::Apu,
    cartridge::Cartridge,
    io::{joypad::JoyPad, serial_transfer::SerialTransfer, timer::Timer},
    ppu::Ppu,
    Mode,
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

#[derive(Debug, PartialEq)]
enum TransferMode {
    Stopped,
    GeneralPurpose,
    HBlank,
}

pub struct Bus {
    cartridge: Cartridge,
    mode: Mode,
    double_speed: bool,
    speed_switch_armed: bool,
    wram_bank: usize,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    hdma: [u8; 4],
    hdma_mode: TransferMode,
    hdma_source: u16,
    hdma_destination: u16,
    hdma_length: u8,
    hdma_counter: u8,
    interrupt_enable: u8,
    interrupt_flag: u8,
    hdma_halted: bool,
    undocumented_cgb_registers: [u8; 3],
    pub joy_pad: JoyPad,
    pub serial_transfer: SerialTransfer,
    pub timer: Timer,
    pub ppu: Ppu,
    pub apu: Apu,
}

impl MemoryAccess for Bus {
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
            0xFF4D | 0xFF4F | 0xFF51..=0xFF56 | 0xFF70 | 0xFF72..=0xFF77 if self.mode != Mode::Color => 0xFF,
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
            0xFF4D | 0xFF4F | 0xFF51..=0xFF56 | 0xFF70 | 0xFF72..=0xFF77 if self.mode != Mode::Color => {}
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

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mode = cartridge.mode();
        let mut bus = Bus {
            cartridge,
            mode,
            double_speed: false,
            speed_switch_armed: false,
            wram_bank: 1,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            hdma: [0; 4],
            hdma_source: 0,
            hdma_destination: 0,
            hdma_mode: TransferMode::Stopped,
            hdma_length: 0xFF,
            hdma_counter: 0,
            interrupt_enable: 0,
            interrupt_flag: 0,
            hdma_halted: false,
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

    pub fn machine_cycle(&mut self, cycles: u32) -> u32 {
        let speed = if self.double_speed { 2 } else { 1 };
        let vram_dma_cycles = self.vram_dma_cycle();
        let cpu_cycles = cycles + vram_dma_cycles * speed;
        let ppu_cycles = cycles / speed + vram_dma_cycles;

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

        return cycles;
    }

    pub fn change_speed(&mut self) {
        if self.speed_switch_armed {
            self.double_speed = !self.double_speed;
        }
        self.speed_switch_armed = false;
    }

    pub fn oam_dma(&mut self, value: u8) {
        let base = (value as u16) << 8;
        for i in 0..0xA0 {
            let byte = self.read_8(base + i);
            self.write_8(0xFE00 + i, byte);
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
            0xFF51 => self.hdma[0] = value,
            0xFF52 => self.hdma[1] = value & 0xF0,
            0xFF53 => self.hdma[2] = value & 0x1F,
            0xFF54 => self.hdma[3] = value & 0xF0,
            0xFF55 => {
                if self.hdma_mode == TransferMode::HBlank {
                    if value & 0x80 == 0 {
                        self.hdma_mode = TransferMode::Stopped;
                    };
                    return;
                }
                let source = ((self.hdma[0] as u16) << 8) | (self.hdma[1] as u16);
                let destination = ((self.hdma[2] as u16) << 8) | (self.hdma[3] as u16) | 0x8000;
                if !(source <= 0x7FF0 || (source >= 0xA000 && source <= 0xDFF0)) {
                    panic!("Invalid HDMA start address {:04X}", source);
                }
                if !(destination >= 0x8000 && destination <= 0x9FF0) {
                    panic!("Invalid HDMA end address {:04X}", destination);
                }

                self.hdma_source = source;
                self.hdma_destination = destination;
                self.hdma_length = value & 0x7F;
                self.hdma_mode = match value & 0x80 != 0 {
                    true => TransferMode::HBlank,
                    false => TransferMode::GeneralPurpose,
                };
            }
            _ => panic!("HDMA does not handle write {:04X}", address),
        };
    }

    fn vram_dma_cycle(&mut self) -> u32 {
        if self.hdma_halted {
            return 0;
        }

        match self.hdma_mode {
            TransferMode::Stopped => 0,
            TransferMode::GeneralPurpose => self.general_purpose_dma(),
            TransferMode::HBlank => self.hblank_dma(),
        }
    }

    fn general_purpose_dma(&mut self) -> u32 {
        let length = self.hdma_length as u32 + 1;
        for _ in 0..length {
            for i in 0..0x10 {
                let byte: u8 = self.read_8(self.hdma_source + i);
                self.ppu.write_8(self.hdma_destination + i, byte);
            }

            self.hdma_source += 0x10;
            self.hdma_destination += 0x10;
            match self.hdma_length == 0 {
                true => self.hdma_length = 0x7F,
                false => self.hdma_length -= 1,
            }
        }

        self.hdma_mode = TransferMode::Stopped;
        return length * 8;
    }

    fn hblank_dma(&mut self) -> u32 {
        if !self.ppu.is_hblanking() {
            return 0;
        }

        for i in 0..0x10 {
            let byte: u8 = self.read_8(self.hdma_source + i);
            self.ppu.write_8(self.hdma_destination + i, byte);
        }

        self.hdma_source += 0x10;
        self.hdma_destination += 0x10;
        match self.hdma_length == 0 {
            true => self.hdma_length = 0x7F,
            false => self.hdma_length -= 1,
        }

        if self.hdma_length == 0x7F {
            self.hdma_mode = TransferMode::Stopped;
        }
        return 8;
    }

    pub fn halt_hblank_dma(&mut self, halted: bool) {
        self.hdma_halted = halted
    }
}
