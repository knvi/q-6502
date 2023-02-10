use crate::{mem::Memory, proc_stat::ProcStat};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pc: u16,    // program counter
    sp: u16,    // stack pointer
    a: u8,      // accumulator
    x: u8,      // x register
    y: u8,      // y register
    p: ProcStat, // processor status

    mem: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.p.clear();

        self.to_owned()
    }
}