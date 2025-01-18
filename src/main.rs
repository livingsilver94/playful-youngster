use std::{fs::File, io, sync::mpsc};

use cpal::traits::{DeviceTrait, HostTrait};
use playful_youngster::{
    emulator::{Emulator, SAMPLE_RATE},
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

const AUDIO_BUFFER_SIZE: usize = 1024;

fn main() -> Result<(), Error> {
    let cartridge = Box::new(File::open("/tmp/cart")?);

    let evtloop = EventLoop::new()?;
    evtloop.run_app(&mut Application::new()?)?;

    Ok(())
}

struct Application {
    emulator: Emulator,

    window: Option<Window>,
    audio: Option<cpal::Stream>,
}

impl Application {
    fn new() -> Result<Self, Error> {
        let (audio_sender, audio_receiver) = mpsc::sync_channel::<(u8, u8)>(AUDIO_BUFFER_SIZE);
        let audio = Self::init_audio(audio_receiver)?;
        Ok(Self {
            emulator: Emulator::new(audio_sender),

            window: None,
            audio,
        })
    }

    fn init_audio(receiver: mpsc::Receiver<(u8, u8)>) -> Result<Option<cpal::Stream>, Error> {
        let play = move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
            for sample in data {
                match receiver.try_recv() {
                    Ok(new_sample) => *sample = new_sample.0,
                    Err(_) => break,
                }
            }
        };

        cpal::default_host()
            .default_output_device()
            .map_or(Ok(None), |device| {
                let config = cpal::StreamConfig {
                    channels: 2,
                    sample_rate: cpal::SampleRate(SAMPLE_RATE),
                    buffer_size: cpal::BufferSize::Fixed(AUDIO_BUFFER_SIZE as u32),
                };
                let stream = device
                    .build_output_stream(
                        &config,
                        play,
                        |err| {
                            eprintln!("{err}");
                        },
                        None,
                    )
                    .unwrap();
                Ok(Some(stream))
            })
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

    #[error("failed to initialize audio system: {0}")]
    Audio(String),
}

impl From<OsError> for Error {
    fn from(value: winit::error::OsError) -> Self {
        Self::Gui(value.to_string())
    }
}

impl From<EventLoopError> for Error {
    fn from(value: winit::error::EventLoopError) -> Self {
        Self::Gui(value.to_string())
    }
}

impl From<cpal::DefaultStreamConfigError> for Error {
    fn from(value: cpal::DefaultStreamConfigError) -> Self {
        Self::Audio(value.to_string())
    }
}
