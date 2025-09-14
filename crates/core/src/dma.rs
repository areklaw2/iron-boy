use crate::{cpu::MemoryInterface, system_bus::SystemMemoryAccess};

#[derive(Debug, PartialEq)]
enum VramTransferMode {
    Stopped,
    GeneralPurpose,
    HBlank,
}
pub struct Dma {
    hdma_mode: VramTransferMode,
    hdma_source: u16,
    hdma_destination: u16,
    hdma_length: u8,
}

impl SystemMemoryAccess for Dma {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF51..=0xFF55 => self.read_hdma(address),
            _ => panic!("DMA does not handle read {:#04X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF46 => self.oam_dma(value),
            0xFF51..=0xFF55 => self.write_hdma(address, value),
            _ => panic!("DMA does not handle write {:#04X}", address),
        }
    }
}

impl Dma {
    pub fn new() -> Self {
        Self {
            hdma_source: 0,
            hdma_destination: 0,
            hdma_mode: VramTransferMode::Stopped,
            hdma_length: 0xFF,
        }
    }

    pub fn cycle<I: MemoryInterface>(&mut self, bus: &mut I) {
        // do hdma logic
        // a bus cycle maybe sometimes to
        bus.m_cycle();
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
