use crate::{bus::Bus, cpu::Cpu};

#[derive(Debug)]
pub struct Gameboy {
    pub mode: GameboyMode,
    pub cpu: Cpu,
    pub bus: Bus,
}

#[derive(Debug, Clone, Copy)]
pub enum GameboyMode {
    Dmg,
    Mgb,
    Sgb,
    Cgb,
}
