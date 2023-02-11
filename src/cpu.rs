use crate::{
    mem::{self, Memory},
    proc_stat::ProcStat,
    op_codes::*
};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    /// program counter
    pc: u16,      
    /// stack pointer      
    sp: u16,  
    /// accumulator          
    a: u8,         
    /// x register     
    x: u8,     
    /// y register         
    y: u8,          
    /// processor status    
    p: ProcStat,        

    /// memory module
    pub mem: Memory,    
}

impl Cpu {
    /// create a new cpu
    pub fn new() -> Self {
        Self::default()
    }

    /// reset cpu to initial state
    pub fn reset(&mut self) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.p.clear();

        self.to_owned()
    }

    /// print contents of registers, pc, sp, and status flags and current instruction
    /// useful when the emulator crashes, you can get a state of the machine
    pub fn debug_print(&self) {
        println!("pc: 0x{:04x}", self.pc);
        println!("sp: 0x{:04x}", self.sp);
        println!("a : 0x{:04x}", self.a);
        println!("x : 0x{:04x}", self.x);
        println!("y : 0x{:04x}", self.y);
        println!("ps: {}", self.p);
        println!("instruction: 0x{:04x}", self.pc);
    }

    /// execute instructions
    pub fn execute(&mut self) {
        loop {
            let instruction = self.fetch_byte();
            match instruction {
                LDA_IM => self.lda_im(),
                LDA_ABS => self.lda_abs(),
                LDA_ABSX => self.lda_absx(),
                LDA_ABSY => self.lda_absy(),
                LDA_ZP => self.lda_zp(),
                LDA_ZPX => self.lda_zpx(),
                LDA_ZPXI => self.lda_zpxi(),
                LDA_ZPYI => self.lda_zpyi(),
                LDX_IM => self.ldx_im(),
                LDX_ABS => self.ldx_abs(),
                LDX_ABSY => self.ldx_absy(),
                LDX_ZP => self.ldx_zp(),
                LDX_ZPY => self.ldx_zpy(),
                LDY_IM => self.ldy_im(),
                LDY_ABS => self.ldy_abs(),
                LDY_ABSX => self.ldy_absx(),
                LDY_ZP => self.ldy_zp(),
                LDY_ZPX => self.ldy_zpx(),
                JSR => self.jsr(),
                NOP => self.nop(),
                _ => {
                    panic!("Unknown instruction: {}", instruction);
                }
            }
        }
    }

    /// fetch a single byte from memory
    fn fetch_memory(&mut self, address: usize) -> u8 {
        self.mem.data[address]
    }

    /// fetch word from memory
    fn fetch_word(&mut self) -> u16 {
        let mut data = self.fetch_memory(self.pc as usize) as u16;
        self.pc += 1;
        data |= (self.fetch_memory(self.pc as usize) as u16) << 8;
        self.pc += 1;
        data
    }

    /// fetch byte from memory
    fn fetch_byte(&mut self) -> u8 {
        if self.pc as usize > mem::MAX_MEM {
            panic!("PC exceeds max memory allocated {}", mem::MAX_MEM);
        }

        let data = self.mem.data[self.pc as usize];
        self.pc += 1;
        data
    }

    /* LOAD A INSTRUCTIONS */

    /// load accumulator immediate
    fn lda_im(&mut self) {
        self.a = self.fetch_byte();
        self.set_flags();
    }

    /// load accumulator absolute
    fn lda_abs(&mut self) {
        let address = self.fetch_word();
        self.a = self.fetch_memory(address as usize);
    }

    /// load accumulator absolute, x index
    fn lda_absx(&mut self) {
        let address = self.fetch_word();
        self.a = self.fetch_memory((address + self.x as u16) as usize);
        self.set_flags();
    }

    /// load accumulator absolute, y index
    fn lda_absy(&mut self) {
        let address = self.fetch_word();
        self.a = self.fetch_memory((address + self.y as u16) as usize);
        self.set_flags();
    }

    /// load accumulator zero page
    fn lda_zp(&mut self) {
        let address = self.fetch_byte();
        self.a = self.mem.read_byte(address as usize);
        self.set_flags();
    }

    /// load accumulator zero page, x index
    fn lda_zpx(&mut self) {
        let address = self.fetch_byte();
        self.a = self.mem.read_byte((address + self.x) as usize);
        self.set_flags();
    }

    /// load accumulator zero page, x index indirect
    fn lda_zpxi(&mut self) {
        let indirect_address = self.fetch_byte() + self.x;
        // & 0xFF will wrap to start of zero page if overflow
        self.a = self.mem.read_byte((indirect_address & 0xFF) as usize);
        self.set_flags();
    }

    /// load accumulator zero page indirect y indexed
    fn lda_zpyi(&mut self) {
        let zero_page_address = self.fetch_byte() + self.y;

        let effective_address = self.mem.read_word(zero_page_address as usize);
        let effective_address_y = effective_address + self.y as u16;

        self.a = self.fetch_memory(effective_address_y as usize);
        self.set_flags();
    }

    /// set zero and negative flags whenever an LDA instruction is executed
    fn set_flags(&mut self) {
        // set zero flag
        self.p.set(ProcStat::Z, self.a == 0);
        self.p
            .set(ProcStat::N, (self.a & 0b10000000) > 0);
    }

    /* LOAD X INSTRUCTIONS */

    /// load x immediate
    fn ldx_im(&mut self) {
        self.x = self.fetch_byte();
        self.set_flags();
    }

    /// load x absolute
    fn ldx_abs(&mut self) {
        let address = self.fetch_word();
        self.x = self.fetch_memory(address as usize);
        self.set_flags();
    }

    /// load x zero page
    fn ldx_zp(&mut self) {
        let address = self.fetch_byte();
        self.x = self.mem.read_byte(address as usize);
        self.set_flags();
    }

    /// load x index y indexed absolute
    fn ldx_absy(&mut self) {
        let address = self.fetch_word();
        self.x = self.fetch_memory((address + self.y as u16) as usize);
        self.set_flags();
    }

    /// load x index y indexed zero page
    fn ldx_zpy(&mut self) {
        let address = self.fetch_byte();
        self.x = self.mem.read_byte((address + self.y) as usize);
        self.set_flags();
    }

    /* LOAD Y INSTRUCTIONS */

    /// load y immediate
    fn ldy_im(&mut self) {
        self.y = self.fetch_byte();
        self.set_flags();
    }

    /// load y absolute
    fn ldy_abs(&mut self) {
        let address = self.fetch_word();
        self.y = self.fetch_memory(address as usize);
        self.set_flags();
    }

    /// load y zero page
    fn ldy_zp(&mut self) {
        let address = self.fetch_byte();
        self.y = self.mem.read_byte(address as usize);
        self.set_flags();
    }

    /// load y index x indexed absolute
    fn ldy_absx(&mut self) {
        let address = self.fetch_word();
        self.y = self.fetch_memory((address + self.x as u16) as usize);
        self.set_flags();
    }

    /// load y index x indexed zero page
    fn ldy_zpx(&mut self) {
        let address = self.fetch_byte();
        self.y = self.mem.read_byte((address + self.x) as usize);
        self.set_flags();
    }

    /// jump to a subroutine by pushing the pc onto the stack and modifying the pc
    fn jsr(&mut self) {
        let sub_address = self.fetch_word();
        self.mem.write_word(self.sp as usize, self.pc - 1);
        self.sp -= 2;
        self.pc = sub_address;
    }

    /// no-op (do nothing)
    fn nop(&mut self) {}
}