use crate::hardware::{
    apu::{
        effect::{Length, Volume},
        TICKS_IN_SAMPLE_RATE,
    },
    MASTER_CLOCK,
};

#[derive(Default)]
pub struct NoiseChannel {
    pub clock_shift: u8,
    /// Whether the LFSR has a large (15 bits) width or short (7 bits).
    pub short_mode: bool,
    pub clock_divider: u8,

    pub volume: Volume,
    pub length: Length<64>,

    /// The [linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register)
    /// involved in the generation of the pseudo-random noise.
    lfsr: u16,
    ticks: u32,
}

impl NoiseChannel {
    pub fn sample(&mut self) -> (u8, u8) {
        if !self.length.tick(TICKS_IN_SAMPLE_RATE) {
            return (0, 0);
        }
        let volume = self.volume.tick(TICKS_IN_SAMPLE_RATE);

        let ticks_to_activate = MASTER_CLOCK / self.frequency();
        let activations = (self.ticks + TICKS_IN_SAMPLE_RATE) / ticks_to_activate;
        for _ in 0..activations {
            self.move_lfsr();
        }
        self.ticks = (self.ticks + TICKS_IN_SAMPLE_RATE) % ticks_to_activate;

        (
            (self.lfsr as u8 & 1) * volume,
            (self.lfsr as u8 & 1) * volume,
        )
    }

    pub fn trigger(&mut self) {
        self.lfsr = 0;
    }

    fn frequency(&self) -> u32 {
        // See https://gbdev.io/pandocs/Audio_Registers.html#ff22--nr43-channel-4-frequency--randomness
        262144 / (self.clock_divider as u32 * (1 << self.clock_shift as u32))
    }

    /// Computes the next shift of the linear-feedback shift register.
    fn move_lfsr(&mut self) {
        // Obtain the 2 least significant bits, that are used as taps.
        let tap1 = self.lfsr & 2 >> 1;
        let tap0 = self.lfsr & 1;
        let xnor = (!(tap1 ^ tap0)) & 1;

        // Set bit 15 with the tap output, and bit 7 as well if in short mode.
        self.lfsr = (self.lfsr & 0x7FFF) | xnor << 15;
        if self.short_mode {
            self.lfsr = (self.lfsr & 0xFF7F) | xnor << 7;
        }

        self.lfsr >>= 1;
    }
}
