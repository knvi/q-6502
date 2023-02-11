mod cpu;
mod mem;
mod op_codes;
mod proc_stat;

use cpu::Cpu;
use op_codes::*;

fn main() {
    let mut cpu = Cpu::new().reset(None);
    // would overflow if ran from reset vector
    // set PC to lower address
    cpu.pc = 0xFFF0;
    // Load a dummy program into mem
    cpu.mem.data[0xFFF0] = LDX_ABS;
    cpu.mem.data[0xFFF1] = 0x80;
    cpu.mem.data[0xFFF2] = 0x44; // 0x4480
    cpu.mem.data[0x4480] = 0x37;
    cpu.mem.data[0xFFF3] = NOP;

    cpu.execute();
    cpu.debug_print();
}
