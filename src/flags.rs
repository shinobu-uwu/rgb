use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InterruptsFlags: u8 {
        const VBLANK = 1 << 0;
        const LCD_STAT = 1 << 1;
        const TIMER   = 1 << 2;
        const SERIAL  = 1 << 3;
        const JOYPAD  = 1 << 4;
    }
}
