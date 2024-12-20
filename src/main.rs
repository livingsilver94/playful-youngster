mod machine;

use std::fs::File;

use machine::Emulator;
use machine::Hardware;

fn main() {
    let mut hw: Hardware<File> = Hardware::new_gb();
    let emu = Emulator::new_gb(&mut hw);
}
