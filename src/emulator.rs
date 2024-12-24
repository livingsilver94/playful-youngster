use winit::{
    error::{EventLoopError, OsError},
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::hardware::{keypad::Button, Cpu, Hardware};

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
            .run(|event, evt_loop| {
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => evt_loop.exit(),
                        WindowEvent::KeyboardInput { event, .. } => {
                            Self::press_key(&mut self.hw, event)
                        }
                        _ => (),
                    }
                }
            })
            .map_err(Error::from)
    }

    fn press_key(hw: &mut Hardware, event: KeyEvent) {
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
        hw.press_key(button, event.state.is_pressed());
    }
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
