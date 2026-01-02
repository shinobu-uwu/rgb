use crate::flags::{LcdControlFlags, LcdStatusFlags};

#[derive(Debug)]
pub struct Ppu {
    oam: [u8; 160],
    vram: [u8; 8 * 1024],
    lcdc: LcdControlFlags,
    stat: u8,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    mode_clock: u32,
    mode: PpuMode,
}

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    #[default]
    OamSearch = 2,
    PixelTransfer = 3,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            vram: [0; 8192],
            oam: [0; 160],
            lcdc: LcdControlFlags::default(),
            stat: 0x85,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            ly: 0,
            lyc: 0,
            mode_clock: 0,
            mode: PpuMode::default(),
        }
    }
}

impl Ppu {
    pub fn read_vram(&self, offset: usize) -> u8 {
        self.vram[offset]
    }

    pub fn read_register(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.lcdc.bits(),
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => unreachable!(),
        }
    }

    pub fn read_oam(&self, offset: usize) -> u8 {
        self.oam[offset]
    }
}
