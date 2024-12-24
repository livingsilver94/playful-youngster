use winit::{
    error::{EventLoopError, OsError},
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::hardware::{Cpu, Hardware, Keypad};

pub struct Emulator {
    cpu: Cpu,
    hw: Hardware,

    event_loop: EventLoop<()>,
    window: Window,
}

impl Emulator {
    pub fn new() -> Result<Self, Error> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Playful Youngster")
            .build(&event_loop)?;
        window.set_cursor_visible(false);

        Ok(Self {
            cpu: Cpu::new(),
            hw: Hardware::new(),

            event_loop,
            window,
        })
    }

    pub fn run(mut self) -> Result<(), Error> {
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
                    } => todo!(),
                    _ => (),
                },
                _ => (),
            })
            .map_err(Error::from)
    }

    fn press_key(keypad: &mut Keypad, event: KeyEvent) {}
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to inizialize the graphics user interface: {0}")]
    Gui(String),
    #[error("failed to allocate memory: {0}")]
    MemoryAlloc(String),
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

impl From<bumpalo::AllocErr> for Error {
    fn from(value: bumpalo::AllocErr) -> Self {
        Self::MemoryAlloc(value.to_string())
    }
}
