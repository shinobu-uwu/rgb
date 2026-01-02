#[derive(Debug, Default)]
pub struct Serial {
    sb: u8,
    sc: u8,
}

impl Serial {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            _ => unreachable!(),
        }
    }
}
