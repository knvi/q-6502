mod mem;
mod cpu;
mod proc_stat;

use cpu::Cpu;

fn main() {
    let cpu = Cpu::new().reset();
}
