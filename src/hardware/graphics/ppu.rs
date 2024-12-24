use pixels::{
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle},
    Pixels, SurfaceTexture,
};

use crate::hardware::graphics::{SCREEN_HEIGHT, SCREEN_WIDTH};

/// The pixel processing unit.
/// In the context of the emulator, it shows a graphical user interface with VRAM's content.
struct Ppu {
    buffer: Pixels,
}

impl Ppu {
    pub fn new<T: HasRawWindowHandle + HasRawDisplayHandle>(
        surface: SurfaceTexture<'_, T>,
    ) -> Self {
        let _ = surface;
        Self {
            buffer: Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface).unwrap(),
        }
    }
}
