use crate::hardware::{
    apu::{effect::Volume, TICKS_IN_SAMPLE_RATE},
    MASTER_CLOCK,
};

#[derive(Default)]
pub struct SquareChannel {
    /// The raw period of the sound wave.
    ///
    /// APU doesn't work with _frequencies_ as audio documentation usually does,
    /// but uses time _periods_.
    ///
    /// Since the raw period is composed of 11 bits, it reaches `2^11 - 1 = 2047`.
    /// This value is way lower than the usual human audible range, which tops at 20 kHz.
    /// A formula is required to covert this raw _period_ value into a proper human frequency.
    pub raw_period: u16,

    /// A timer that, when it reaches 64, makes the channel to turn off automatically.
    pub length_timer: u8,

    /// Sets the duty cycle pattern. It's the index of a row of [DUTY_CYCLES].
    pub duty_cycle_pattern: u8,

    /// Sets whether the sound is on or off at this time, depending on the current duty cycle pattern.
    /// It's the index of a column of [DUTY_CYCLES].
    duty_cycle_position: u8,

    volume: Volume,

    /// A counter involved in the wave generation, that decrements at each clock tick.
    /// When it reaches zero, its value is recalculated to a starting value and [Self::duty_cycle_position]
    /// advances.
    frequency_timer: u16,

    /// Remembers the subtick the generator was at, in the previous sample.
    /// Another way to see it is the current phase of the wave.
    subtick: u32,

    /// Control bits.
    controls: Controls,
}

impl SquareChannel {
    pub const fn sample(&mut self) -> (u8, u8) {
        self.follow_wave();
        (0, 0)
    }

    /// Returns the real frequency, in Hz, computed from the raw period.
    const fn frequency(&self) -> u32 {
        // See https://gbdev.io/pandocs/Audio_Registers.html#ff13--nr13-channel-1-period-low-write-only
        131072 / (2048 - self.raw_period as u32)
    }

    /// Whether the square wave, at a given instant, is high (returns 1) or low (returns 0).
    const fn wave_amplitude(&self) -> u8 {
        DUTY_CYCLES[self.duty_cycle_pattern as usize][self.duty_cycle_position as usize]
    }

    const fn follow_wave(&mut self) {
        const DUTY_CYCLE_RESOLUTION: u32 = DUTY_CYCLES[0].len() as u32;
        let subtick_ratio = MASTER_CLOCK / (self.frequency() * 8);

        self.subtick = self
            .subtick
            .wrapping_add(TICKS_IN_SAMPLE_RATE / subtick_ratio);
        self.duty_cycle_position =
            ((self.duty_cycle_position as u32 + self.subtick) % DUTY_CYCLE_RESOLUTION) as u8;
    }
}

#[derive(Default)]
struct Controls(bitflags::BitFlags8);

impl Controls {
    const fn is_triggered(&self) -> bool {
        self.0.get(7)
    }

    const fn is_length_timer_enabled(&self) -> bool {
        self.0.get(6)
    }
}

/// Possible duty cycle patterns for the APU.
const DUTY_CYCLES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [0, 0, 0, 0, 0, 0, 1, 1], // 25%
    [0, 0, 0, 0, 1, 1, 1, 1], // 50%
    [1, 1, 1, 1, 1, 1, 0, 0], // 75%
];
