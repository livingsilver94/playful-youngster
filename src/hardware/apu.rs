mod effect;
mod square;

use bitflags::BitFlags8;

use square::SquareChannel;

use crate::hardware;

/// Sample rate of all sound.
const SAMPLE_RATE: u32 = 44100;

/// Size, in bytes, of the sound buffer.
const BUFFER_SIZE: u32 = 1024;
/// Size, in bytes, of one sound sample.
const SAMPLE_SIZE: u32 = 1;

const MAX_PERIOD: u16 = 2047;

/// Number of buffers to pass to the sound card, per second, to play the sound.
// The division by is because sound is stereo: there are 2 channels.
const BUFFERS_PER_SECOND: u32 = SAMPLE_RATE / ((BUFFER_SIZE / SAMPLE_SIZE) / 2);
const TICKS_IN_SAMPLE_RATE: u32 = hardware::MASTER_CLOCK / SAMPLE_RATE;

// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

#[derive(Default)]
pub struct Apu {
    volume: MasterVolume,
    ch1: SquareChannel,
    ch2: SquareChannel,

    /// Number of clock ticks that have passed.
    /// Tick count is used to synchronize [Self::frame_sequencer].
    ticks: u32,
    /// Clock generator for Sweep, Envelope and Length.
    /// It counts from 0 to 7, and for each of these values, one or more effect
    /// generators are activated.
    frame_sequencer: u8,
}

impl Apu {
    pub fn new() -> Self {
        Default::default()
    }

    /// Advances the internal state of the APU and produces one audio sample.
    pub fn tick(&mut self, ticks: u8) {
        // TODO: pulse channels are ticked every 4 CPU cycles. Emulate that.
        if !self.volume.is_audio_on() {
            return;
        }
        self.ticks += ticks as u32;
        if self.ticks < TICKS_IN_SAMPLE_RATE {
            return;
        }
        self.ticks -= TICKS_IN_SAMPLE_RATE;

        let (left, right) = [self.ch1.sample(), self.ch2.sample()]
            .iter()
            .fold((0, 0), |sum, sample| (sum.0 + sample.0, sum.1 + sample.1));

        if self.ticks > 8192 {
            match self.frame_sequencer {
                0 => todo!(),
                1 => todo!(),
                2 => todo!(),
                3 => todo!(),
                4 => todo!(),
                5 => todo!(),
                6 => todo!(),
                7 => todo!(),
                _ => unreachable!(),
            }
            self.frame_sequencer = (self.frame_sequencer + 1) % 8;
        }
    }

    fn read_register(&self, idx: usize) -> u8 {
        match idx {
            0x01 => (self.ch1.duty_cycle_pattern << 6) & self.ch1.length_timer,
            0x06 => (self.ch2.duty_cycle_pattern << 6) & self.ch2.length_timer,
            _ => unreachable!(),
        }
    }

    fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            0x01 => {
                self.ch1.duty_cycle_pattern = (val & 0b11000000) >> 6;
                self.ch1.length_timer = val & 0b00111111;
            }
            0x06 => {
                self.ch2.duty_cycle_pattern = (val & 0b11000000) >> 6;
                self.ch2.length_timer = val & 0b00111111;
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct MasterVolume(BitFlags8);

impl MasterVolume {
    fn is_audio_on(&self) -> bool {
        self.0.get(7)
    }

    fn is_ch4_on(&self) -> bool {
        self.0.get(3)
    }

    fn is_ch3_on(&self) -> bool {
        self.0.get(2)
    }

    fn is_ch2_on(&self) -> bool {
        self.0.get(1)
    }

    fn is_ch1_on(&self) -> bool {
        self.0.get(0)
    }
}

impl From<u8> for MasterVolume {
    fn from(value: u8) -> Self {
        Self(BitFlags8::from(value))
    }
}
