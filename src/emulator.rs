use crate::hardware::{self, keypad::Button, Cartridge, Cpu, Hardware};
use std::{sync::mpsc, thread, time};

/// Target framerate (aka FPS) for the emulator.
const FRAMERATE: u32 = 60;
pub use hardware::apu::SAMPLE_RATE;

pub struct Emulator {
    cpu: Cpu,
    hw: Hardware,
}

impl Emulator {
    pub fn new(audio_buffer: mpsc::SyncSender<(u8, u8)>) -> Self {
        Self {
            cpu: Cpu::new(),
            hw: Hardware::new(audio_buffer),
        }
    }

    pub fn insert_cartridge(&mut self, cart: Cartridge) {
        self.hw.insert_cartridge(cart);
    }

    pub fn set_pressed(&mut self, button: Button, pressed: bool) {
        self.hw.keypad.set_pressed(button, pressed);
    }

    pub fn process_frame(&mut self) {
        const TICKS_PER_FRAME: u32 = hardware::MASTER_CLOCK / FRAMERATE;
        const FRAMETIME: f32 = 1.0 / (FRAMERATE as f32);

        let mut total_ticks = 0;
        let begin = time::Instant::now();
        while total_ticks < TICKS_PER_FRAME {
            let ticks = self.cpu.tick(&mut self.hw);
            self.hw.timer.tick(ticks);
            self.hw.apu.tick(ticks);
            total_ticks += ticks as u32;
        }
        thread::sleep(time::Duration::from_secs_f32(FRAMETIME).saturating_sub(begin.elapsed()));
    }
}
