mod emulator;
mod hardware;

use winit::event_loop::EventLoop;

use std::fs::File;

use hardware::Cartridge;

use crate::emulator::{Emulator, Error};

fn main() -> Result<(), Error> {
    let cartridge = Box::new(File::open("/tmp/cart")?);

    let mut emu = Emulator::new()?;
    emu.insert_cartridge(Cartridge::new_from_header(cartridge)?);

    let evtloop = EventLoop::new()?;
    evtloop.run_app(&mut emu)?;

    Ok(())
}
