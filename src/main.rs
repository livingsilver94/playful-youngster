use std::{fs::File, io};

use playful_youngster::{
    emulator::Emulator,
    hardware::{keypad::Button, Cartridge},
};
use winit::{
    application::ApplicationHandler,
    error::{EventLoopError, OsError},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

fn main() -> Result<(), Error> {
    let cartridge = Box::new(File::open("/tmp/cart")?);

    let mut emu = Emulator::new();
    emu.insert_cartridge(Cartridge::new_from_header(cartridge)?);

    let evtloop = EventLoop::new()?;
    evtloop.run_app(&mut Application::new(emu))?;

    Ok(())
}

struct Application {
    emulator: Emulator,
    window: Option<Window>,
}

impl Application {
    fn new(emu: Emulator) -> Self {
        Self {
            emulator: emu,
            window: None,
        }
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, evtloop: &ActiveEventLoop) {
        evtloop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        if self.window.is_some() {
            return;
        }

        let window = evtloop
            .create_window(WindowAttributes::default().with_title("Playful Youngster"))
            .unwrap();
        window.set_cursor_visible(false);
        self.window = Some(window);
    }

    fn window_event(&mut self, evtloop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
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
                self.emulator.set_pressed(button, event.state.is_pressed());
            }
            WindowEvent::CloseRequested => evtloop.exit(),
            _ => (),
        }
    }

    fn about_to_wait(&mut self, evtloop: &ActiveEventLoop) {
        evtloop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        self.emulator.process_frame();
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
