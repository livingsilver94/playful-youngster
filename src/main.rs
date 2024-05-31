mod machine;

use machine::Emulator;
use machine::Hardware;

fn main() {
    let hw = Hardware::default();
    let emu = Emulator::new_gb(&hw);
}
