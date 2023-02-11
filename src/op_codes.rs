// load accumulator immediate
pub const LDA_IM: u8 = 0xA9;
// load accumulator absolute
pub const LDA_ABS: u8 = 0xAD;
// load accumulator absolute x indexed
pub const LDA_ABSX: u8 = 0xBD;
// load accumulator absolute y indexed
pub const LDA_ABSY: u8 = 0xB9;
// load accumulator zero page
pub const LDA_ZP: u8 = 0xA5;
// load accumulator zero page, x index
pub const LDA_ZPX: u8 = 0xB5;
// load accumulator zero page x indexed indirect
pub const LDA_ZPXI: u8 = 0xA1;
// load accumulator zero page y indexed indirect
pub const LDA_ZPYI: u8 = 0xB1;
// no-op
pub const NOP: u8 = 0xEA;
// jump subroutine
pub const JSR: u8 = 0x20;