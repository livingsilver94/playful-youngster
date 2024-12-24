mod emulator;
mod hardware;

use std::fs::File;

use hardware::Cartridge;

use crate::emulator::{Emulator, Error};

fn main() -> Result<(), Error> {
    let cartridge = File::open("/tmp/cart").unwrap();

    let emu = Emulator::new()?;
    //machine.insert_cartridge(Cartridge::new_from_header(cartridge).unwrap());

    emu.run();
    Ok(())
}
