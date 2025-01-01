use std::io;

use winit::{
    application::ApplicationHandler,
    error::{EventLoopError, OsError},
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::hardware::{keypad::Button, Cartridge, Cpu, Hardware};

/// Target framerate (aka FPS) for the emulator.
const FRAMERATE: u32 = 60;

pub struct Emulator {
    // Emulated hardware.
    cpu: Cpu,
    hw: Hardware,

    // OS facilities.
    window: Option<Window>,
}

impl Emulator {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            cpu: Cpu::new(),
            hw: Hardware::new(),

            window: None,
        })
    }

    pub fn insert_cartridge(&mut self, cart: Cartridge) {
        self.hw.insert_cartridge(cart);
    }

    fn press_key(&mut self, event: KeyEvent) {
        let button = match event.physical_key {
            PhysicalKey::Code(code) => match code {
                KeyCode::KeyW => Button::Up,
                KeyCode::KeyS => Button::Down,
                KeyCode::KeyA => Button::Left,
                KeyCode::KeyD => Button::Down,
                KeyCode::Semicolon => Button::A,
                KeyCode::Quote => Button::B,
                KeyCode::Enter => Button::Start,
                KeyCode::Space => Button::Select,
                _ => return,
            },
            _ => return,
        };
        self.hw.press_key(button, event.state.is_pressed());
    }
}

impl ApplicationHandler for Emulator {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let window = event_loop
            .create_window(WindowAttributes::default().with_title("Playful Youngster"))
            .unwrap();
        window.set_cursor_visible(false);
        self.window = Some(window);
    }

    fn window_event(&mut self, evtloop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => self.press_key(event),
            WindowEvent::CloseRequested => evtloop.exit(),
            _ => (),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to inizialize the graphics user interface: {0}")]
    Gui(String),

    #[error("I/O error")]
    Io(#[from] io::Error),
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
