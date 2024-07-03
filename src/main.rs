mod machine;

use machine::Emulator;
use machine::Hardware;

fn main() {
    let mut hw = Hardware::new_gb();
    let emu = Emulator::new_gb(&mut hw);
}
