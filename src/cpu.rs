use crate::{
    mem::{self, Memory},
    proc_stat::ProcStat,
    op_codes::*
};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pc: u16,    // program counter
    sp: u16,    // stack pointer
    a: u8,      // accumulator
    x: u8,      // x register
    y: u8,      // y register
    p: ProcStat, // processor status

    pub mem: Memory,
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

    // print contents of registers, pc, sp, and status flags
    fn debug_print(&self) {
        println!("pc: 0x{:04x}", self.pc);
        println!("sp: 0x{:04x}", self.sp);
        println!("a : 0x{:04x}", self.a);
        println!("x : 0x{:04x}", self.x);
        println!("y : 0x{:04x}", self.y);
        println!("ps: {}", self.p);
    }

    pub fn execute(&mut self) {
        loop {
            let instruction = self.fetch_and_increment();
            match instruction {
                LDA_IM => self.lda_im(),
                LDA_ABS => self.lda_abs(),
                LDA_ABSX => self.lda_absx(),
                LDA_ZP => self.lda_zp(),
                LDA_ZPX => self.lda_zpx(),
                NOP => self.nop(),
                _ => {
                    panic!("Unknown instruction: {}", instruction);
                }
            }
        }
    }

    // fetch a single byte from zero page memory
    fn fetch_zero_page(&mut self, address: usize) -> u8 {
        if self.pc as usize > mem::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", mem::MAX_MEM);
        }

        self.mem.mem[address]
    }

    // fetch a single byte from memory
    fn fetch_memory(&mut self, address: usize) -> u8 {
        self.mem.mem[address]
    }

    // fetch word from memory
    fn fetch_word(&mut self) -> u16 {
        let mut data = self.fetch_memory(self.pc as usize) as u16;
        self.pc += 1;
        data |= (self.fetch_memory(self.pc as usize) as u16) << 8;
        self.pc += 1;
        data
    }

    fn fetch(&mut self, address: usize) -> u8 {
        if self.pc as usize > mem::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", mem::MAX_MEM);
        }

        self.mem.mem[address]
    }

    fn fetch_and_increment(&mut self) -> u8 {
        if self.pc as usize > mem::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", mem::MAX_MEM);
        }

        let data = self.mem.mem[self.pc as usize];
        self.pc += 1;
        data
    }

    /// LOAD A INSTRUCTIONS

    // load accumulator immediate
    fn lda_im(&mut self) {
        self.a = self.fetch_and_increment();
        self.lda_set_flags();
    }

    // load accumulator absolute
    fn lda_abs(&mut self) {
        let address = self.fetch_word();
        self.a = self.fetch_memory(address as usize);
    }

    // load accumulator absolute, x index
    fn lda_absx(&mut self) {
        let address = self.fetch_word();
        self.a = self.fetch_memory((address + self.x as u16) as usize);
        self.lda_set_flags();
    }

    // load accumulator zero page
    fn lda_zp(&mut self) {
        let address = self.fetch_and_increment();
        self.a = self.fetch_zero_page(address as usize);
        self.lda_set_flags();
    }

    // load accumulator zero page, x index
    fn lda_zpx(&mut self) {
        let address = self.fetch_and_increment();
        self.a = self.fetch_zero_page((address + self.x) as usize);
        self.lda_set_flags();
    }

    // set zero and negative flags whenever an LDA instruction is executed
    fn lda_set_flags(&mut self) {
        // set zero flag
        self.p.set(ProcStat::Z, bool::from(self.a == 0));
        self.p
            .set(ProcStat::N, bool::from((self.a & 0b10000000) > 0));
    }

    // no-op
    fn nop(&mut self) {}
}