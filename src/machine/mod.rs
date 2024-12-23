mod apu;
mod cartridge;
mod cpu;
mod graphics;
mod keypad;
mod memory;
mod timer;

use std::io::{Read, Seek};

use cartridge::Cartridge;
use cpu::Cpu;
use graphics::Gpu;
use keypad::Keypad;
use memory::Mmu;
use timer::Timer;
use winit::{
    dpi::PhysicalSize,
    error::{EventLoopError, OsError},
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct Hardware<R: Read + Seek> {
    gpu: Gpu,
    keypad: Keypad,
    timer: Timer,
    cartridge: Option<Cartridge<R>>,
}

impl<R: Read + Seek> Hardware<R> {
    pub fn new_gb() -> Self {
        Self {
            gpu: Gpu::new_gb(),
            keypad: Keypad::new(),
            timer: Timer::default(),
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge<R>) {
        self.cartridge = Some(cartridge);
    }
}

pub struct Emulator<'a> {
    cpu: Cpu<'a>,

    event_loop: EventLoop<()>,
    window: Window,
}

impl<'a> Emulator<'a> {
    pub fn new<R: Read + Seek>(hw: &'a mut Hardware<R>) -> Result<Self, Error> {
        let mmu = Mmu::new_gb(&mut hw.gpu, &mut hw.keypad, &mut hw.timer);

        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Playful Youngster")
            .with_min_inner_size(PhysicalSize {
                width: graphics::SCREEN_WIDTH * 2,
                height: graphics::SCREEN_HEIGHT * 2,
            })
            .build(&event_loop)?;
        window.set_cursor_visible(false);

        Ok(Self {
            cpu: Cpu::new(mmu),
            event_loop,
            window,
        })
    }

    pub fn run(self) -> Result<(), Error> {
        self.event_loop
            .run(|event, evt_loop| match event {
                Event::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    WindowEvent::CloseRequested => evt_loop.exit(),
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    } => Self::process_event(self., event),
                    _ => (),
                },
                _ => (),
            })
            .map_err(Error::from)
    }

    fn process_event(hw: &'a mut Hardware<R>, event: KeyEvent) {
        let _ = hw;
        //match event {}
    }

    fn press_key(&self) {}
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to inizialize the graphics user interface: {0}")]
    Gui(String),
}

impl From<OsError> for Error {
    fn from(value: OsError) -> Self {
        Self::Gui(value.to_string())
    }
}

impl From<EventLoopError> for Error {
    fn from(value: EventLoopError) -> Self {
        Self::Gui(value.to_string())
    }
}
