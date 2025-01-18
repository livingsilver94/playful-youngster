mod effect;
mod noise;
mod square;
mod wave;

use std::sync::mpsc;

use bitflags::BitFlags8;

use square::SquareChannel;

use crate::hardware::{
    self,
    apu::{noise::NoiseChannel, wave::WaveChannel},
};

/// Sample rate of all sound.
pub const SAMPLE_RATE: u32 = 44100;

/// Size, in bytes, of one sound sample.
const SAMPLE_SIZE: u32 = 1;

const MAX_PERIOD: u16 = 2047;

const TICKS_IN_SAMPLE_RATE: u32 = hardware::MASTER_CLOCK / SAMPLE_RATE;

// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

pub struct Apu {
    volume: MasterVolume,
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,

    /// Number of clock ticks that have passed.
    /// Tick count is used to synchronize [Self::frame_sequencer].
    ticks: u32,
    buffer: mpsc::SyncSender<(u8, u8)>,
}

impl Apu {
    pub fn new(buffer: mpsc::SyncSender<(u8, u8)>) -> Self {
        Self {
            volume: Default::default(),
            ch1: Default::default(),
            ch2: Default::default(),
            ch3: Default::default(),
            ch4: Default::default(),

            ticks: Default::default(),
            buffer,
        }
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

        let sample = [
            self.ch1.sample(),
            self.ch2.sample(),
            self.ch3.sample(),
            self.ch4.sample(),
        ]
        .iter()
        .fold((0, 0), |sum, sample| (sum.0 + sample.0, sum.1 + sample.1));
        let _ = self.buffer.try_send(sample);
    }

    pub fn read_register(&self, idx: usize) -> u8 {
        match idx {
            0x0 => self.ch1.sweep.as_register(),
            0x1 => self.ch1.duty_cycle_pattern << 6,
            0x2 => self.ch1.volume.as_register(),
            0x3 => self.ch1.raw_period as u8,
            0x4 => {
                (self.ch1.length.enabled as u8) << 6 | ((self.ch1.raw_period & 0x0700) >> 8) as u8
            }
            0x6 => self.ch2.duty_cycle_pattern << 6,
            0x7 => self.ch2.volume.as_register(),
            0x8 => self.ch2.raw_period as u8,
            0x9 => {
                (self.ch2.length.enabled as u8) << 6 | ((self.ch2.raw_period & 0x0700) >> 8) as u8
            }
            0xA => (self.ch3.enabled as u8) << 7,
            0xB => unreachable!(),
            0xC => u8::from(self.ch3.volume) << 5,
            0xD => self.ch3.raw_period as u8,
            0xE => {
                (self.ch3.length.enabled as u8) << 6 | ((self.ch3.raw_period & 0x0700) >> 8) as u8
            }
            0x11 => self.ch4.volume.as_register(),
            0x12 => {
                self.ch4.clock_shift << 4
                    | (self.ch4.short_mode as u8) << 3
                    | self.ch4.clock_divider
            }
            0x13 => (self.ch4.length.enabled as u8) << 6,
            0x20..=0x2F => self.ch3.wave_ram[idx - 0x20],
            _ => unreachable!(),
        }
    }

    pub fn write_register(&mut self, idx: usize, val: u8) {
        match idx {
            0x0 => self.ch1.sweep.set_from_register(val),
            0x1 => {
                self.ch1.duty_cycle_pattern = (val & 0b11000000) >> 6;
                self.ch1.length.set_timer(val & 0b11111);
            }
            0x2 => self.ch1.volume.set_from_register(val),
            0x3 => self.ch1.raw_period = (self.ch1.raw_period & 0xFF00) | val as u16,
            0x4 => {
                self.ch1.length.enabled = (val & 0b1000000) != 0;
                self.ch1.raw_period = self.ch1.raw_period & 0x00FF | ((val & 0b111) as u16) << 8;
                if val & 0b10000000 != 0 {
                    self.ch1.trigger();
                }
            }
            0x6 => {
                self.ch2.duty_cycle_pattern = (val & 0b11000000) >> 6;
                self.ch2.length.set_timer(val & 0b11111);
            }
            0x7 => self.ch2.volume.set_from_register(val),
            0x8 => self.ch2.raw_period = (self.ch2.raw_period & 0xFF00) | val as u16,
            0x9 => {
                self.ch2.length.enabled = (val & 0b1000000) != 0;
                self.ch2.raw_period = self.ch2.raw_period & 0x00FF | ((val & 0b111) as u16) << 8;
                if val & 0b10000000 != 0 {
                    self.ch2.trigger();
                }
            }
            0xA => self.ch3.enabled = val & 0b10000000 != 0,
            0xB => self.ch3.length.set_timer(val),
            0xC => self.ch3.volume = ((val & 0b01100000) >> 5).into(),
            0xD => self.ch3.raw_period = (self.ch3.raw_period & 0xFF00) | val as u16,
            0xE => {
                self.ch3.length.enabled = (val & 0b1000000) != 0;
                self.ch3.raw_period = self.ch3.raw_period & 0x00FF | ((val & 0b111) as u16) << 8;
                if val & 0b10000000 != 0 {
                    self.ch3.trigger();
                }
            }
            0x10 => self.ch4.length.set_timer(val & 0b111111),
            0x11 => self.ch4.volume.set_from_register(val),
            0x12 => {
                self.ch4.clock_shift = (val & 0b11110000) >> 4;
                self.ch4.short_mode = val & 0b1000 != 0;
                self.ch4.clock_divider = val & 0b111;
            }
            0x13 => {
                self.ch4.length.enabled = val & 0b1000000 != 0;
                if val & 0b10000000 != 0 {
                    self.ch4.trigger();
                }
            }
            0x20..=0x2F => self.ch3.wave_ram[idx - 0x20] = val,
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
