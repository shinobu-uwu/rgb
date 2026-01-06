use std::ops::Add;

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

#[derive(Copy, Clone)]
pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TCycle(pub u32);

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

    pub const fn flag(&self, flag: CpuFlag) -> bool {
        (self.f & flag as u8) > 0
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

    pub const fn set_flag(&mut self, flag: CpuFlag, value: bool) {
        let mask = flag as u8;

        if value {
            self.f |= mask;
        } else {
            self.f &= !mask
        }

        self.f &= 0xF0;
    }
}

impl Add for TCycle {
    type Output = Self;

    fn add(self, other: TCycle) -> TCycle {
        TCycle(self.0 + other.0)
    }
}

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) -> TCycle {
        let opcode = bus.read(self.registers.pc);
        self.registers.pc += 1;

        match opcode {
            0x00 => TCycle(1),
            0x01 => {
                let val = bus.read16(self.registers.pc);
                self.registers.set_bc(val);
                self.registers.pc += 2;
                TCycle(12)
            }
            0x02 => {
                bus.write(self.registers.bc(), self.registers.a);
                TCycle(8)
            }
            0x03 => {
                self.registers.set_bc(self.registers.bc() + 1);
                TCycle(8)
            }
            0x04 => {
                let old = self.registers.b;
                let new = old.wrapping_add(1);
                self.registers.b = new;

                self.registers.set_flag(CpuFlag::Z, new == 0);
                self.registers.set_flag(CpuFlag::N, false);
                self.registers
                    .set_flag(CpuFlag::H, (old & 0xF) + (1 & 0xF) > 0xF);
                TCycle(4)
            }
            0x05 => {
                self.registers.b -= 1;
                TCycle(4)
            }
            0x06 => {
                let val = bus.read(self.registers.pc);
                self.registers.b = val;
                TCycle(8)
            }
            0x07 => {}
            _ => panic!("Invalid opcode {opcode}"),
        }
    }
}
