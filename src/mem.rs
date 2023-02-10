const MAX_MEM: usize = 1024 * 64;

#[derive(Debug, Clone)]
pub struct Memory {
    pub mem: [u8; MAX_MEM],
}

impl Default for Memory {
    fn default() -> Self {
        Memory { mem: [0; MAX_MEM] }
    }
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }
}