use std::{cell::RefCell, rc::Rc};

use getset::Getters;

use crate::{GbSpeed, cartridge::Cartridge, memory::Memory, ppu::Ppu, system_bus::SystemMemoryAccess, t_cycles};

const OAM_DMA_T_CYCLES: u16 = 640;

#[derive(Debug, PartialEq)]
enum VramTransferMode {
    Stopped,
    GeneralPurpose,
    HBlank,
}

#[derive(Getters)]
pub struct Dma {
    oam_dma_source_address: u16,
    oam_dma_pending: bool,
    #[getset(get = "pub")]
    oam_dma_active: bool,
    oam_dma_cycles: u16,

    hdma_mode: VramTransferMode,
    hdma_source: u16,
    hdma_destination: u16,
    hdma_length: u8,
    speed: Rc<RefCell<GbSpeed>>,
}

impl SystemMemoryAccess for Dma {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF46 => self.read_oam_dma(),
            0xFF51..=0xFF55 => self.read_hdma(address),
            _ => panic!("DMA does not handle read {:#04X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF46 => self.write_oam_dma(value),
            0xFF51..=0xFF55 => self.write_hdma(address, value),
            _ => panic!("DMA does not handle write {:#04X}", address),
        }
    }
}

impl Dma {
    pub fn new(speed: Rc<RefCell<GbSpeed>>) -> Self {
        Self {
            oam_dma_source_address: 0xFF00,
            oam_dma_pending: false,
            oam_dma_active: false,
            oam_dma_cycles: 0,

            hdma_source: 0,
            hdma_destination: 0,
            hdma_mode: VramTransferMode::Stopped,
            hdma_length: 0xFF,
            speed,
        }
    }

    pub fn read_oam_dma(&self) -> u8 {
        (self.oam_dma_source_address >> 8) as u8
    }

    pub fn write_oam_dma(&mut self, value: u8) {
        self.oam_dma_source_address = (value as u16) << 8;
        self.oam_dma_pending = true;
    }

    pub fn oam_dma_cycle(&mut self, cartridge: &Cartridge, memory: &Memory, ppu: &mut Ppu) {
        if self.oam_dma_pending {
            self.oam_dma_cycles = OAM_DMA_T_CYCLES;
            self.oam_dma_pending = false;
            return;
        }

        if self.oam_dma_cycles == 0 {
            self.oam_dma_active = false;
            return;
        }
        self.oam_dma_active = true;

        let byte = match self.oam_dma_source_address {
            0x0000..=0x7FFF => cartridge.read_8(self.oam_dma_source_address),
            0x8000..=0x9FFF => ppu.read_8(self.oam_dma_source_address),
            0xA000..=0xBFFF => cartridge.read_8(self.oam_dma_source_address),
            0xC000..=0xDFFF => memory.read_8(self.oam_dma_source_address),
            0xE000..=0xFFFF => 0xFF,
        };
        ppu.write_8(0xFE00 | (self.oam_dma_source_address & 0x00FF), byte);

        self.oam_dma_source_address += 1;
        self.oam_dma_cycles -= t_cycles(*self.speed.borrow()) as u16;
    }

    fn read_hdma(&self, address: u16) -> u8 {
        match address {
            0xFF51..=0xFF54 => 0xFF,
            0xFF55 => ((self.hdma_mode == VramTransferMode::Stopped) as u8) << 7 | self.hdma_length,
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
                VramTransferMode::HBlank => {
                    if value & 0x80 == 0 {
                        self.hdma_mode = VramTransferMode::Stopped;
                    }
                }
                VramTransferMode::Stopped => {
                    self.hdma_mode = match value & 0x80 != 0 {
                        true => VramTransferMode::HBlank,
                        false => VramTransferMode::GeneralPurpose,
                    };
                    self.hdma_length = (value & 0x7F) + 1;
                }
                VramTransferMode::GeneralPurpose => panic!("Cannot cancel General Purpose DMA"),
            },
            _ => panic!("HDMA does not handle write {:04X}", address),
        };
    }

    // fn vram_dma_cycle(&mut self, cpu_halted: bool) -> u32 {
    //     match self.hdma_mode {
    //         VramTransferMode::Stopped => 0,
    //         VramTransferMode::GeneralPurpose => self.general_purpose_dma(),
    //         VramTransferMode::HBlank => self.hblank_dma(cpu_halted),
    //     }
    // }

    // fn general_purpose_dma(&mut self) -> u32 {
    //     let length = self.hdma_length as u32;
    //     for _ in 0..length {
    //         for _ in 0..0x10 {
    //             let b: u8 = self.read_8(self.hdma_source);
    //             self.ppu.write_8(self.hdma_destination | 0x8000, b);
    //             self.hdma_source += 1;
    //             self.hdma_destination += 1;
    //         }

    //         if self.hdma_length != 0 {
    //             self.hdma_length -= 1;
    //         }
    //     }

    //     self.hdma_mode = VramTransferMode::Stopped;
    //     length * 32
    // }

    // fn hblank_dma(&mut self, halted: bool) -> u32 {
    //     if !self.ppu.is_hblanking() || halted {
    //         return 0;
    //     }

    //     for _ in 0..0x10 {
    //         let b: u8 = self.read_8(self.hdma_source);
    //         self.ppu.write_8(self.hdma_destination | 0x8000, b);
    //         self.hdma_source += 1;
    //         self.hdma_destination += 1;
    //     }

    //     if self.hdma_length != 0 {
    //         self.hdma_length -= 1;
    //     }

    //     if self.hdma_length == 0 {
    //         self.hdma_mode = VramTransferMode::Stopped;
    //     }

    //     32
    // }
}
