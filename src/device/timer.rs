#[derive(Debug, Default)]
pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn step(&mut self, cycles: u8) -> bool {
        let mut interrupt = false;

        for _ in 0..cycles {
            let old_div = self.div;
            self.div = self.div.wrapping_add(1);

            if (self.tac & 0x04) != 0 {
                let bit = match self.tac & 0x03 {
                    0 => 9, // 4096 Hz (1024 cycles)
                    1 => 3, // 262144 Hz (16 cycles)
                    2 => 5, // 65536 Hz (64 cycles)
                    3 => 7, // 16384 Hz (256 cycles)
                    _ => unreachable!(),
                };

                if (old_div & (1 << bit)) != 0 && (self.div & (1 << bit)) == 0 {
                    let (new_tima, overflow) = self.tima.overflowing_add(1);

                    if overflow {
                        self.tima = self.tma;
                        interrupt = true;
                    } else {
                        self.tima = new_tima;
                    }
                }
            }
        }

        interrupt
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac | 0xF8, // top 5 bits are unused
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val & 0x07, // only bottom 3 bits are writable
            _ => unreachable!(),
        }
    }
}
