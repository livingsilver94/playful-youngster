use crate::hardware::apu::effect::Length;

// TODO: terminate implementation.

/// Plays audio samples stored in memory.
pub struct WaveChannel {
    /// Memory area containing 32 samples of 4 bits each.
    wave_ram: [u8; 16],
    volume: Volume,
    length: Length<256>,
}

#[derive(Default)]
enum Volume {
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
