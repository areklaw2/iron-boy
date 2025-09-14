use std::cell::RefCell;
use std::rc::Rc;

use getset::{Getters, MutGetters};

use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::cpu::MemoryInterface;
use crate::dma::Dma;
use crate::interrupts::Interrupts;
use crate::joypad::JoyPad;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::serial_transfer::SerialTransfer;
use crate::speed_switch::SpeedSwitch;
use crate::timer::Timer;
use crate::{GbMode, GbSpeed, t_cycles};

pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

pub trait SystemMemoryAccess {
    fn read_8(&self, address: u16) -> u8;

    fn write_8(&mut self, address: u16, value: u8);
}

#[derive(Getters, MutGetters)]
pub struct SystemBus {
    gb_mode: GbMode,
    speed_switch: SpeedSwitch,
    undocumented_cgb_registers: [u8; 3],
    interrupts: Interrupts,
    dma: Dma,
    memory: Memory,
    cartridge: Cartridge,
    #[getset(get = "pub", get_mut = "pub")]
    joy_pad: JoyPad,
    serial_transfer: SerialTransfer,
    timer: Timer,
    #[getset(get = "pub", get_mut = "pub")]
    ppu: Ppu,
    #[getset(get = "pub", get_mut = "pub")]
    pub apu: Apu,
    total_m_cycles: u64,
    #[getset(get = "pub")]
    total_t_cycles: u64,
}

impl SystemMemoryAccess for SystemBus {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_8(address),
            0x8000..=0x9FFF => self.ppu.read_8(address),
            0xA000..=0xBFFF => self.cartridge.read_8(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.memory.read_8(address),
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.memory.read_8(address),
            0xFE00..=0xFE9F => match !self.dma.oam_dma_active() {
                true => self.ppu.read_8(address),
                false => 0xFF,
            },
            0xFF00 => self.joy_pad.read_8(address),
            0xFF01..=0xFF02 => self.serial_transfer.read_8(address),
            0xFF04..=0xFF07 => self.timer.read_8(address),
            0xFF0F => *self.interrupts.interrupt_flag().borrow() & 0b0001_1111,
            0xFF10..=0xFF3F => self.apu.read_8(address),
            0xFF40..=0xFF4B => self.ppu.read_8(address),
            0xFF4D | 0xFF4F | 0xFF51..=0xFF56 | 0xFF70 | 0xFF72..=0xFF77 if self.gb_mode != GbMode::Color => 0xFF,
            0xFF4D => self.speed_switch.read_8(address),
            0xFF4F => self.ppu.read_8(address),
            0xFF50 => todo!("Set to non-zero to disable boot ROM"),
            0xFF51..=0xFF55 => self.dma.read_8(address),
            0xFF56 => 0xFF, //todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.read_8(address),
            0xFF70 => self.memory.read_8(address),
            0xFF72..=0xFF73 => self.undocumented_cgb_registers[address as usize - 0xFF72],
            0xFF75 => self.undocumented_cgb_registers[2] | 0x8F,
            0xFF76..=0xFF77 => self.apu.read_8(address),
            0xFF80..=0xFFFE => self.memory.read_8(address),
            0xFFFF => *self.interrupts.interrupt_enable(),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write_8(address, value),
            0x8000..=0x9FFF => self.ppu.write_8(address, value),
            0xA000..=0xBFFF => self.cartridge.write_8(address, value),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.memory.write_8(address, value),
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.memory.write_8(address, value),
            0xFE00..=0xFE9F => {
                if !self.dma.oam_dma_active() {
                    self.ppu.write_8(address, value)
                }
            }
            0xFF00 => self.joy_pad.write_8(address, value),
            0xFF01..=0xFF02 => self.serial_transfer.write_8(address, value),
            0xFF04..=0xFF07 => self.timer.write_8(address, value),
            0xFF0F => *self.interrupts.interrupt_flag().borrow_mut() = value,
            0xFF10..=0xFF3F => self.apu.write_8(address, value),
            0xFF40..=0xFF45 => self.ppu.write_8(address, value),
            0xFF46 => self.dma.write_8(address, value),
            0xFF47..=0xFF4B => self.ppu.write_8(address, value),
            0xFF4D | 0xFF4F | 0xFF51..=0xFF56 | 0xFF70 | 0xFF72..=0xFF77 if self.gb_mode != GbMode::Color => {}
            0xFF4D => self.speed_switch.write_8(address, value),
            0xFF4F => self.ppu.write_8(address, value),
            0xFF50 => {}
            0xFF51..=0xFF55 => self.dma.write_8(address, value),
            0xFF56 => {} //todo!("Infrared Comms"),
            0xFF68..=0xFF6C => self.ppu.write_8(address, value),
            0xFF70 => self.memory.write_8(address, value),
            0xFF72..=0xFF73 => self.undocumented_cgb_registers[address as usize - 0xFF72] = value,
            0xFF75 => self.undocumented_cgb_registers[2] = value,
            0xFF76..=0xFF77 => self.apu.write_8(address, value),
            0xFF80..=0xFFFE => self.memory.write_8(address, value),
            0xFFFF => {
                self.interrupts.set_interrupt_enable(value);
            }
            _ => {}
        }
    }
}

impl MemoryInterface for SystemBus {
    fn load_8(&mut self, address: u16, with_cycles: bool) -> u8 {
        if with_cycles {
            self.m_cycle();
        }
        self.read_8(address)
    }

    fn store_8(&mut self, address: u16, value: u8, with_cycles: bool) {
        if with_cycles {
            self.m_cycle();
        }
        self.write_8(address, value);
    }

    fn m_cycle(&mut self) {
        //let speed = if self.speed_switch.double_speed() { 2 } else { 1 };
        //let vram_cycles = self.vram_dma_cycle(cpu_halted);
        //let cpu_cycles = cycles + vram_cycles * speed;
        //let ppu_cycles = cycles / speed + vram_cycles;
        let t_cycles = t_cycles(*self.speed_switch.speed().borrow());
        self.total_t_cycles = self.total_t_cycles.wrapping_add(t_cycles as u64);
        self.total_m_cycles = self.total_m_cycles + 1;

        self.dma.oam_dma_cycle(&self.cartridge, &self.memory, &mut self.ppu);
        self.timer.cycle();
        self.ppu.cycle();
        self.apu.cycle();
    }

    fn total_m_cycles(&self) -> u64 {
        self.total_m_cycles as u64
    }

    fn pending_interrupt(&self) -> u8 {
        let interrupt_flag = *self.interrupts.interrupt_flag().borrow();
        let interrupt_enable = self.interrupts.interrupt_enable();
        return interrupt_flag & interrupt_enable & 0x1F;
    }

    fn clear_interrupt(&mut self, interrupt_bit: u8) {
        *self.interrupts.interrupt_flag().borrow_mut() &= !(1 << interrupt_bit);
    }

    fn speed(&self) -> GbSpeed {
        *self.speed_switch.speed().borrow()
    }

    fn change_speed(&mut self) {
        self.speed_switch.change_speed();
    }
}

impl SystemBus {
    pub fn new(cartridge: Cartridge) -> Self {
        let mode = cartridge.mode();
        let speed = Rc::new(RefCell::new(GbSpeed::Normal));
        let interrupt_flag = Rc::new(RefCell::new(0));
        let mut bus = SystemBus {
            gb_mode: mode,
            memory: Memory::new(),
            undocumented_cgb_registers: [0; 3],
            speed_switch: SpeedSwitch::new(speed.clone()),
            dma: Dma::new(speed.clone()),
            cartridge,
            interrupts: Interrupts::new(interrupt_flag.clone()),
            joy_pad: JoyPad::new(interrupt_flag.clone()),
            serial_transfer: SerialTransfer::new(interrupt_flag.clone()),
            timer: Timer::new(speed.clone(), interrupt_flag.clone()),
            ppu: Ppu::new(mode, speed.clone(), interrupt_flag),
            apu: Apu::new(speed),
            total_m_cycles: 0,
            total_t_cycles: 0,
        };

        bus.set_hardware_registers();
        bus
    }

    fn set_hardware_registers(&mut self) {
        self.write_8(0xFF04, 0);
        self.write_8(0xFF05, 0);
        self.write_8(0xFF06, 0);
        self.write_8(0xFF07, 0xF8);
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
}
