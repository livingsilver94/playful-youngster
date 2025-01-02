use std::process::Command;
use std::process::ExitStatus;

const BOOTROM_PATH: &str = "bootrom";

fn main() -> Result<(), ExitStatus> {
    let bootrom = Command::new("make")
        .current_dir(BOOTROM_PATH)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    if !bootrom.success() {
        return Err(bootrom);
    }
    println!("cargo::rerun-if-changed={BOOTROM_PATH}");
    Ok(())
}
