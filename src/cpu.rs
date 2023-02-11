use crate::{
    mem::{self, Memory},
    proc_stat::ProcStat,
    op_codes::*
};

#[derive(Debug, Default, Clone)]
pub struct Cpu {
    /// program counter
    pub pc: u16,      
    /// stack pointer      
    pub sp: u16,  
    /// accumulator          
    pub a: u8,         
    /// x register     
    pub x: u8,     
    /// y register         
    pub y: u8,          
    /// processor status    
    pub p: ProcStat,        

    /// memory module
    pub mem: Memory,    
}

impl Cpu {
    /// create a new cpu
    pub fn new() -> Self {
        Self::default()
    }

    /// reset cpu to initial state
    /// an optional address can be passed to set the program counter
    /// if no address is passed, the program counter is set to 0xFFFC
    /// this is the default reset vector for the NES
    /// https://wiki.nesdev.com/w/index.php/CPU_power_up_state
    pub fn reset(&mut self, address: Option<u16>) -> Self {
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.p.clear();

        // read 0xFFFC and 0xFFFD and
        // jump to that address for instructions
        if let Some(address) = address {
            self.mem.write_word(self.pc as usize, address);
            self.pc = self.mem.read_word(0xFFFC);
        }

        self.to_owned()
    }

    /// load a program into the cpu's memory at a given address
    pub fn load_program(&mut self, address: usize, program: Vec<u8>) {
        todo!()
    }

    /// print contents of registers, pc, sp, and status flags and current instruction
    /// useful when the emulator crashes, you can get a state of the machine
    pub fn debug_print(&mut self) {
        println!("pc: 0x{:04x}", self.pc);
        println!("sp: 0x{:04x}", self.sp);
        println!("a : 0x{:04x}", self.a);
        println!("x : 0x{:04x}", self.x);
        println!("y : 0x{:04x}", self.y);
        println!("ps: {}", self.p);
        println!(
            "current instruction: 0x{:02X}",
            self.mem.read_byte(self.pc as usize)
        );
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
                LSR_ACC => self.lsr_acc(),
                LSR_ABS => self.lsr_abs(),
                LSR_ZP => self.lsr_zp(),
                LSR_ABSX => self.lsr_absx(),
                LSR_ZPX => self.lsr_zpx(),
                PHA => self.pha(),
                PHP => self.php(),
                PLA => self.pla(),
                PLP => self.plp(),
                ORA_IM => self.ora_im(),
                ORA_ABS => self.ora_abs(),
                ORA_ABSX => self.ora_absx(),
                ORA_ABSY => self.ora_absy(),
                ORA_ZP => self.ora_zp(),
                ORA_ZPX => self.ora_zpx(),
                ORA_ZPXI => self.ora_zpxi(),
                ORA_ZPYI => self.ora_zpyi(),
                ANDA_IM => self.anda_im(),
                ANDA_ABS => self.anda_abs(),
                ANDA_ABSX => self.anda_absx(),
                ANDA_ABSY => self.anda_absy(),
                ANDA_ZP => self.anda_zp(),
                ANDA_ZPX => self.anda_zpx(),
                ANDA_ZPXI => self.anda_zpxi(),
                ANDA_ZPYI => self.anda_zpyi(),
                EORA_IM => self.eor_im(),
                EORA_ABS => self.eor_abs(),
                EORA_ABSX => self.eor_absx(),
                EORA_ABSY => self.eor_absy(),
                EORA_ZP => self.eor_zp(),
                EORA_ZPX => self.eor_zpx(),
                EORA_ZPXI => self.eor_zpxi(),
                EORA_ZPYI => self.eor_zpyi(),
                TAX => self.tax(),
                TAY => self.tay(),
                TSX => self.tsx(),
                TXA => self.txa(),
                TXS => self.txs(),
                TYA => self.tya(),
                JSR => self.jsr(),
                RTS => self.rts(),
                NOP => break,
                _ => {
                    self.debug_print();
                    panic!("reason: unrecognized instruction");
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

    /* logical shift right instructions */

    /// logical shift right accumulator
    fn lsr_acc(&mut self) {
        self.a >>= 1;
        self.set_flags();

        self.set_carry_flag((self.a & 0b1) > 0);
    }

    /// logical shift right absolute
    fn lsr_abs(&mut self) {
        let address = self.fetch_word() as usize;
        let mut data = self.mem.read_byte(address);

        let carry = data & 1;
        data >>= 1;

        self.mem.write_byte(address, data);

        self.p.set(ProcStat::Z, data == 0);
        self.p.set(ProcStat::N, false);
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right zero page
    fn lsr_zp(&mut self) {
        let address = self.fetch_byte() as usize;
        let mut data = self.mem.read_byte(address);

        let carry = data & 1;
        data >>= 1;

        self.mem.write_byte(address, data);

        self.p.set(ProcStat::Z, data == 0);
        self.p.set(ProcStat::N, false);
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right absolute, x index
    fn lsr_absx(&mut self) {
        let address = self.fetch_word() as usize;
        let mut data = self.mem.read_byte(address + self.x as usize);

        let carry = data & 1;
        data >>= 1;
        self.mem.write_byte(address + self.x as usize, data >> 1);

        self.p.set(ProcStat::Z, data == 0);
        self.p.set(ProcStat::N, false);
        self.set_carry_flag(carry > 0);
    }

    /// logical shift right zero page, x index
    fn lsr_zpx(&mut self) {
        let address = self.fetch_byte() as usize;
        let data = self.mem.read_byte(address + self.x as usize);

        self.mem.write_byte(address + self.x as usize, data >> 1);

        self.set_flags();
        self.set_carry_flag((data & 1) > 0);
    }

    /* PUSH INSTRUCTIONS */

    /// push accumulator onto stack
    fn pha(&mut self) {
        self.mem.write_byte(self.sp as usize, self.a);
        self.sp -= 1;
    }

    /// push processor status onto stack
    fn php(&mut self) {
        self.mem.write_byte(self.sp as usize, self.p.bits());
        self.sp -= 1;
    }

    /* POP INSTRUCTIONS */

    /// pop accumulator from stack
    fn pla(&mut self) {
        self.sp += 1;
        self.a = self.mem.read_byte(self.sp as usize);
        self.set_flags();
    }

    /// pop processor status from stack
    fn plp(&mut self) {
        self.sp += 1;
        self.p = ProcStat::from_bits_truncate(self.mem.read_byte(self.sp as usize));
    }

    /* ORA INSTRUCTIONS */

    /// or accumulator with immediate
    fn ora_im(&mut self) {
        self.a |= self.fetch_byte();
        self.set_flags();
    }

    /// or accumulator with absolute
    fn ora_abs(&mut self) {
        let address = self.fetch_word();
        self.a |= self.fetch_memory(address as usize);
        self.set_flags();
    }

    /// or accumulator with zero page
    fn ora_zp(&mut self) {
        let address = self.fetch_byte();
        self.a |= self.mem.read_byte(address as usize);
        self.set_flags();
    }

    /// or accumulator with absolute, x index
    fn ora_absx(&mut self) {
        let address = self.fetch_word();
        self.a |= self.fetch_memory((address + self.x as u16) as usize);
        self.set_flags();
    }

    /// or accumulator with absolute, y index
    fn ora_absy(&mut self) {
        let address = self.fetch_word();
        self.a |= self.fetch_memory((address + self.y as u16) as usize);
        self.set_flags();
    }

    /// or accumulator with zero page, x index
    fn ora_zpx(&mut self) {
        let address = self.fetch_byte();
        self.a |= self.mem.read_byte((address + self.x) as usize);
        self.set_flags();
    }

    /// or accumulator with indirect, x index
    fn ora_zpxi(&mut self) {
        let address = self.fetch_byte();
        let eff_address = self.mem.read_word((address + self.x) as usize);
        self.a |= self.fetch_memory(eff_address as usize);
        self.set_flags();
    }

    /// or accumulator with indirect, y index
    fn ora_zpyi(&mut self) {
        let address = self.fetch_byte();
        let eff_address = self.mem.read_word(address as usize) + self.y as u16;
        self.a |= self.fetch_memory(eff_address as usize);
        self.set_flags();
    }

    /* ANDA instructions */

    /// and accumulator with immediate
    fn anda_im(&mut self) {
        self.a &= self.fetch_byte();
        self.set_flags();
    }

    /// and accumulator with absolute
    fn anda_abs(&mut self) {
        let address = self.fetch_word();
        self.a &= self.fetch_memory(address as usize);
        self.set_flags();
    }

    /// and accumulator with zero page
    fn anda_zp(&mut self) {
        let address = self.fetch_byte();
        self.a &= self.mem.read_byte(address as usize);
        self.set_flags();
    }

    /// and accumulator with absolute, x index
    fn anda_absx(&mut self) {
        let address = self.fetch_word();
        self.a &= self.fetch_memory((address + self.x as u16) as usize);
        self.set_flags();
    }

    /// and accumulator with absolute, y index
    fn anda_absy(&mut self) {
        let address = self.fetch_word();
        self.a &= self.fetch_memory((address + self.y as u16) as usize);
        self.set_flags();
    }

    /// and accumulator with zero page, x index
    fn anda_zpx(&mut self) {
        let address = self.fetch_byte();
        self.a &= self.mem.read_byte((address + self.x) as usize);
        self.set_flags();
    }

    /// and accumulator with indirect, x index
    fn anda_zpxi(&mut self) {
        let address = self.fetch_byte();
        let eff_address = self.mem.read_word((address + self.x) as usize);
        self.a &= self.fetch_memory(eff_address as usize);
        self.set_flags();
    }

    /// and accumulator with indirect, y index
    fn anda_zpyi(&mut self) {
        let address = self.fetch_byte();
        let eff_address = self.mem.read_word(address as usize) + self.y as u16;
        self.a &= self.fetch_memory(eff_address as usize);
        self.set_flags();
    }

    /* EOR instructions */

    /// exclusive or accumulator with immediate
    fn eor_im(&mut self) {
        self.a ^= self.fetch_byte();
        self.set_flags();
    }

    /// exclusive or accumulator with absolute
    fn eor_abs(&mut self) {
        let address = self.fetch_word();
        self.a ^= self.fetch_memory(address as usize);
        self.set_flags();
    }

    /// exclusive or accumulator with zero page
    fn eor_zp(&mut self) {
        let address = self.fetch_byte();
        self.a ^= self.mem.read_byte(address as usize);
        self.set_flags();
    }

    /// exclusive or accumulator with absolute, x index
    fn eor_absx(&mut self) {
        let address = self.fetch_word();
        self.a ^= self.fetch_memory((address + self.x as u16) as usize);
        self.set_flags();
    }

    /// exclusive or accumulator with absolute, y index
    fn eor_absy(&mut self) {
        let address = self.fetch_word();
        self.a ^= self.fetch_memory((address + self.y as u16) as usize);
        self.set_flags();
    }

    /// exclusive or accumulator with zero page, x index
    fn eor_zpx(&mut self) {
        let address = self.fetch_byte();
        self.a ^= self.mem.read_byte((address + self.x) as usize);
        self.set_flags();
    }

    /// exclusive or accumulator with indirect, x index
    fn eor_zpxi(&mut self) {
        let address = self.fetch_byte();
        let eff_address = self.mem.read_word((address + self.x) as usize);
        self.a ^= self.fetch_memory(eff_address as usize);
        self.set_flags();
    }

    /// exclusive or accumulator with indirect, y index
    fn eor_zpyi(&mut self) {
        let address = self.fetch_byte();
        let eff_address = self.mem.read_word(address as usize) + self.y as u16;
        self.a ^= self.fetch_memory(eff_address as usize);
        self.set_flags();
    }

    /* TRANSFER INSTRUCTIONS */

    /// transfer accumulator to x register
    fn tax(&mut self) {
        self.x = self.a;
        self.p.set(ProcStat::Z, self.x == 0);
        self.p.set(ProcStat::N, (self.x & 0x80) > 0);
    }

    /// transfer accumulator to y register
    fn tay(&mut self) {
        self.y = self.a;
        self.p.set(ProcStat::Z, self.y == 0);
        self.p.set(ProcStat::N, (self.y & 0x80) > 0);
    }

    /// transfer x register to accumulator
    fn txa(&mut self) {
        self.a = self.x;
        self.p.set(ProcStat::Z, self.a == 0);
        self.p.set(ProcStat::N, (self.a & 0x80) > 0);
    }

    /// transfer y register to accumulator
    fn tya(&mut self) {
        self.a = self.y;
        self.p.set(ProcStat::Z, self.a == 0);
        self.p.set(ProcStat::N, (self.a & 0x80) > 0);
    }

    /// transfer x register to stack pointer
    fn txs(&mut self) {
        self.sp = 0x0100 | self.x as u16;
    }

    /// transfer stack pointer to x register
    fn tsx(&mut self) {
        self.x = (self.sp & 0x00FF) as u8;

        self.p.set(ProcStat::Z, self.x == 0);
        self.p.set(ProcStat::N, (self.x & 0x80) > 0);
    }

    /// sets the carry bit if flag is true in processor status register
    fn set_carry_flag(&mut self, carry: bool) {
        self.p.set(ProcStat::C, carry);
    }

    /// jump to a subroutine by pushing the pc onto the stack and modifying the pc
    fn jsr(&mut self) {
        let sub_address = self.fetch_word();
        self.mem.write_word(self.sp as usize, self.pc - 1);
        self.sp -= 2;
        self.pc = sub_address;
    }

    /// return from subroutine, taking PC from stack and continuing before the jump
    fn rts(&mut self) {
        self.sp += 1;
        let pch = self.mem.read_byte(self.sp as usize);
        self.sp += 1;
        let pcl = self.mem.read_byte(self.sp as usize);
        self.pc = (((pch as u16) << 8) | pcl as u16) + 1;
    }

    /// no-op (do nothing)
    fn nop(&mut self) {}
}