// load accumulator immediate
pub const LDA_IM: u8 = 0xA9;
// load accumulator absolute
pub const LDA_ABS: u8 = 0xAD;
// load accumulator absolute, x index
pub const LDA_ABSX: u8 = 0xBD;
// load accumulator zero page
pub const LDA_ZP: u8 = 0xA5;
// load accumulator zero page, x index
pub const LDA_ZPX: u8 = 0xB5;
// no-op
pub const NOP: u8 = 0xEA;