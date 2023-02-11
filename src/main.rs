mod mem;
mod cpu;
mod op_codes;
mod proc_stat;

use cpu::Cpu;
use op_codes::*;

fn main() {
    let mut cpu = Cpu::new().reset();
    cpu.mem.mem[0xFFFC] = NOP;
    cpu.execute();
}
