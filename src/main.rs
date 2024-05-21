mod machine;

use crate::machine::cpu;
use crate::machine::memory;

fn main() {
    let mut mmu = memory::Mmu::new();
    let cpu = cpu::Cpu::new(&mut mmu);
}
