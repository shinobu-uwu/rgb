use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InterruptsFlags: u8 {
        const VBLANK = 1 << 0;
        const LCD_STAT = 1 << 1;
        const TIMER = 1 << 2;
        const SERIAL = 1 << 3;
        const JOYPAD = 1 << 4;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LcdControlFlags: u8 {
        const BG_WINDOW_ENABLE = 1 << 0;
        const OBJ_ENABLE = 1 << 1;
        const OBJ_SIZE = 1 << 2;
        const BG_TILE_MAP = 1 << 3;
        const BG_WINDOW_TILES = 1 << 4;
        const WINDOW_ENABLE = 1 << 5;
        const WINDOW_TILE_MAP = 1 << 6;
        const LCD_PPU_ENABLE = 1 << 7;
    }
}

impl Default for LcdControlFlags {
    fn default() -> Self {
        LcdControlFlags::LCD_PPU_ENABLE
            | LcdControlFlags::BG_WINDOW_TILES
            | LcdControlFlags::BG_WINDOW_ENABLE
    }
}
