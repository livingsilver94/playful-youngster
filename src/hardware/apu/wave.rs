use crate::hardware::apu::effect::Length;

// TODO: terminate implementation.

/// Plays audio samples stored in memory.
#[derive(Default)]
pub struct WaveChannel {
    pub enabled: bool,

    /// The raw period of the sound wave.
    ///
    /// APU doesn't work with _frequencies_ as audio documentation usually does,
    /// but uses time _periods_.
    ///
    /// Since the raw period is composed of 11 bits, it reaches `2^11 - 1 = 2047`.
    /// This value is way lower than the usual human audible range, which tops at 20 kHz.
    /// A formula is required to covert this raw _period_ value into a proper human frequency.
    pub raw_period: u16,

    pub volume: Volume,
    pub length: Length<256>,

    /// Memory area containing 32 samples of 4 bits each.
    pub wave_ram: [u8; 16],
}

impl WaveChannel {
    pub fn sample(&mut self) -> (u8, u8) {
        todo!()
    }

    /// Returns the real frequency, in Hz, computed from the raw period.
    const fn frequency(&self) -> u32 {
        // See https://gbdev.io/pandocs/Audio_Registers.html#ff1d--nr33-channel-3-period-low-write-only
        65536 / (2048 - self.raw_period as u32)
    }

    pub(crate) fn trigger(&self) -> () {
        todo!()
    }
}

#[derive(Clone, Copy, Default)]
pub enum Volume {
    #[default]
    Pattern0,
    Pattern1,
    Pattern2,
    Pattern3,
}

impl Volume {
    /// Determines the amount of bits the volume is shifted.
    fn shift_amount(&self) -> u8 {
        match self {
            Self::Pattern0 => 4,
            Self::Pattern1 => 0,
            Self::Pattern2 => 1,
            Self::Pattern3 => 2,
        }
    }
}

impl From<u8> for Volume {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Pattern0,
            1 => Self::Pattern1,
            2 => Self::Pattern2,
            3 => Self::Pattern3,
            _ => unreachable!(),
        }
    }
}

impl From<Volume> for u8 {
    fn from(value: Volume) -> Self {
        match value {
            Volume::Pattern0 => 0,
            Volume::Pattern1 => 1,
            Volume::Pattern2 => 2,
            Volume::Pattern3 => 3,
        }
    }
}
