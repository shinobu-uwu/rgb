use crate::{
    device::{audio::Audio, joypad::Joypad, ppu::Ppu, serial::Serial, timer::Timer},
    flags::InterruptsFlags,
};

#[derive(Debug)]
pub struct Bus {
    pub rom: [u8; 32 * 1024],
    pub wram: [u8; 8 * 1024],
    pub hram: [u8; 127],
    pub wave_ram: [u8; 16],
    pub serial: Serial,
    pub joypad: Joypad,
    pub timer: Timer,
    pub ppu: Ppu,
    pub audio: Audio,
    pub interrupt_flags: InterruptsFlags,
    pub interrupt_enable: InterruptsFlags,
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0..0x8000 => self.rom[addr as usize],
            0x8000..0xA000 => self.ppu.read_vram(addr as usize - 0x8000),
            0xA000..0xE000 => self.wram[addr as usize - 0xA000],
            0xE000..0xFE00 => self.wram[addr as usize - 0xE000],
            0xFE00..0xFEA0 => self.ppu.read_oam(addr as usize - 0xFE00),
            0xFEA0..0xFF00 => 0x00,
            0xFF00 => self.joypad.read(),
            0xFF01..0xFF03 => self.serial.read(addr),
            0xFF03 => 0xFF, // unmapped I/O
            0xFF04..0xFF08 => self.timer.read(addr),
            0xFF08..0xFF0F => 0xFF, // unmapped I/O
            0xFF0F => self.interrupt_flags.bits(),
            0xFF10..0xFF27 => self.audio.read(addr),
            0xFF27..0xFF30 => 0xFF, // unmapped I/O
            0xFF30..0xFF40 => self.wave_ram[addr as usize - 0xFF30],
            0xFF46 => 0xFF, // DMA is write-only
            0xFF40..0xFF4C => self.ppu.read_register(addr),
            0xFF4C..0xFF4E => 0xFF, // for now let's ignore KEY0/1
            0xFF4E => 0xFF,         // unmapped I/O
            0xFF4F => todo!(),
            0xFF80..0xFFFF => self.hram[addr as usize - 0xFF80],
            0xFFFF => self.interrupt_enable.bits(),
            _ => 0xFF,
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr + 1) as u16;

        (high << 8) | low
    }

    pub fn write(&mut self, addr: u16, val: u8) {}

    pub fn write16(&mut self, addr: u16, val: u16) {}
}
