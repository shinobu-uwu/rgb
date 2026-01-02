use crate::bus::Bus;

#[derive(Debug, Clone, Default)]
pub struct Cpu {
    pub registers: Registers,
}

#[derive(Debug, Clone, Default)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Registers {
    pub const fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub const fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub const fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub const fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub const fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;
    }

    pub const fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub const fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub const fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }
}

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) {
        let opcode = bus.read(self.registers.pc);
        self.registers.pc += 1;

        match opcode {
            0x00 => {
                // NOP
            }
            0x1 => {
                let val = bus.read16(self.registers.pc);
                self.registers.set_bc(val);
                self.registers.pc += 2;
            }
            _ => panic!("Invalid opcode {opcode}"),
        }
    }
}
