pub const MAX_MEM: usize = 1024 * 64;

#[derive(Debug, Clone)]
pub struct Memory {
    pub data: [u8; MAX_MEM],
}

impl Default for Memory {
    fn default() -> Self {
        Memory { data: [0; MAX_MEM] }
    }
}

impl Memory {
    /// write a word (2 bytes) to memory
    pub fn write_word(&mut self, address: usize, data: u16) {
        self.write_byte(address, (data & 0xFF) as u8);
        self.write_byte(address + 1, (data >> 8) as u8);
    }

    /// write a byte to memory
    pub fn write_byte(&mut self, address: usize, data: u8) {
        self.data[address] = data;
    }

    /// read a byte from memory
    pub fn read_byte(&mut self, address: usize) -> u8 {
        self.data[address]
    }

    /// read a word (2 bytes) from memory
    pub fn read_word(&mut self, address: usize) -> u16 {
        let mut data = self.read_byte(address) as u16;
        data |= u16::from(self.read_byte(address + 1)) << 8;
        data
    }
}