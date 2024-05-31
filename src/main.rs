mod machine;

use crate::machine::cpu;
use crate::machine::keypad;
use crate::machine::memory;

fn main() {
    let key = keypad::Keypad::new();
    let mut mmu = memory::Mmu::new_gb(&key);
    let mut cpu = cpu::Cpu::new(&mut mmu);
}
