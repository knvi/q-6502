/*
    processor status
    "The flags register, also called processor status or just P,
    is one of the six architectural registers on the 6502 family CPU.
    It is composed of six one-bit registers; instructions modify one or more bits and leave others unchanged."
    - https://www.nesdev.org/wiki/Status_flags
*/

use core::fmt;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct ProcStat: u8 {
        const N = 0b1000_0000; // negative
        const V = 0b0100_0000; // overflow
        const B = 0b0001_0000; // break
        const D = 0b0000_1000; // decimal
        const I = 0b0000_0100; // interrupt disable
        const Z = 0b0000_0010; // zero
        const C = 0b0000_0001; // carry
    }
}

impl ProcStat {
    pub fn clear(&mut self) -> &mut Self {
        self.bits = 0;
        self
    }
}

impl fmt::Display for ProcStat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.bits)
    }
}