use std::{cell::RefCell, rc::Rc};

use getset::Getters;

use crate::{
    GbSpeed,
    cartridge::Cartridge,
    memory::Memory,
    ppu::{Ppu, registers::PpuMode},
    system_bus::SystemMemoryAccess,
    t_cycles,
};

const OAM_DMA_T_CYCLES: u16 = 640;
const VRAM_DMA_BLOCK_SIZE: u8 = 16;

#[derive(Debug, PartialEq)]
enum VramDmaMode {
    Stopped,
    HdmaPending,
    HdmaActive,
    GdmaActive,
}

#[derive(Getters)]
pub struct Dma {
    oam_dma_source_address: u16,
    oam_dma_pending: bool,
    #[getset(get = "pub")]
    oam_dma_active: bool,
    oam_dma_cycles: u16,
    vram_dma_mode: VramDmaMode,
    vram_dma_source_address: u16,
    vram_dma_destination_address: u16,
    vram_dma_length: u8,
    vram_bytes_transferred: u8,
    speed: Rc<RefCell<GbSpeed>>,
}

impl SystemMemoryAccess for Dma {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF46 => (self.oam_dma_source_address >> 8) as u8,
            0xFF51..=0xFF54 => 0xFF,
            0xFF55 => ((self.vram_dma_mode == VramDmaMode::Stopped) as u8) << 7 | self.vram_dma_length,
            _ => panic!("DMA does not handle read {:#04X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF46 => {
                self.oam_dma_source_address = (value as u16) << 8;
                self.oam_dma_pending = true;
            }
            0xFF51 => self.vram_dma_source_address = (self.vram_dma_source_address & 0x00FF) | (value as u16) << 8,
            0xFF52 => self.vram_dma_source_address = (self.vram_dma_source_address & 0xFF00) | (value & 0xF0) as u16,
            0xFF53 => self.vram_dma_destination_address = (self.vram_dma_destination_address & 0x00FF) | ((value & 0x1F) as u16) << 8,
            0xFF54 => self.vram_dma_destination_address = (self.vram_dma_destination_address & 0xFF00) | (value & 0xF0) as u16,
            0xFF55 => match self.vram_dma_mode {
                VramDmaMode::HdmaActive | VramDmaMode::HdmaPending => {
                    if value & 0x80 == 0 {
                        self.vram_dma_mode = VramDmaMode::Stopped;
                    }
                }
                VramDmaMode::Stopped => {
                    self.vram_dma_mode = match value & 0x80 != 0 {
                        true => VramDmaMode::HdmaPending,
                        false => VramDmaMode::GdmaActive,
                    };
                    self.vram_dma_length = (value & 0x7F) + 1;
                }
                VramDmaMode::GdmaActive => panic!("Cannot cancel General Purpose DMA"),
            },
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
            vram_dma_source_address: 0,
            vram_dma_destination_address: 0,
            vram_dma_mode: VramDmaMode::Stopped,
            vram_dma_length: 0xFF,
            vram_bytes_transferred: 0,
            speed,
        }
    }

    pub fn cycle(&mut self, cartridge: &Cartridge, memory: &Memory, ppu: &mut Ppu, cpu_halted: bool) {
        self.oam_dma_cycle(cartridge, memory, ppu);
        self.vram_dma_cycle(cartridge, memory, ppu, cpu_halted);
    }

    fn oam_dma_cycle(&mut self, cartridge: &Cartridge, memory: &Memory, ppu: &mut Ppu) {
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

    fn vram_dma_cycle(&mut self, cartridge: &Cartridge, memory: &Memory, ppu: &mut Ppu, cpu_halted: bool) {
        match self.vram_dma_mode {
            VramDmaMode::Stopped => return,
            VramDmaMode::HdmaPending => {
                if !cpu_halted && ppu.mode() == PpuMode::HBlank {
                    self.vram_dma_mode = VramDmaMode::HdmaActive;
                } else {
                    return;
                }
            }
            _ => {}
        }

        for _ in 0..2 {
            let byte = match self.vram_dma_source_address {
                0x0000..=0x7FFF => cartridge.read_8(self.vram_dma_source_address),
                0xA000..=0xBFFF => cartridge.read_8(self.vram_dma_source_address),
                0xC000..=0xDFFF => memory.read_8(self.vram_dma_source_address),
                0x8000..=0x9FFF | 0xE000..=0xFFFF => 0xFF,
            };
            ppu.write_8(0x8000 | (self.vram_dma_destination_address & 0x1FFF), byte);

            self.vram_dma_source_address = self.vram_dma_source_address.wrapping_add(1);
            self.vram_dma_destination_address = self.vram_dma_destination_address.wrapping_add(1);
            self.vram_bytes_transferred += 1;
        }

        if self.vram_bytes_transferred == VRAM_DMA_BLOCK_SIZE {
            self.vram_dma_length -= 1;
            self.vram_bytes_transferred = 0;
            if self.vram_dma_mode == VramDmaMode::HdmaActive {
                self.vram_dma_mode = VramDmaMode::HdmaPending
            }
        }

        if self.vram_dma_length == 0 || self.vram_dma_destination_address == 0x0000 {
            self.vram_dma_mode = VramDmaMode::Stopped;
        }
    }

    // pub fn vram_dma_active(&self) -> bool {
    //     matches!(self.vram_dma_mode, VramDmaMode::GdmaActive | VramDmaMode::HdmaActive)
    // }
}
