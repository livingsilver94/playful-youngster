use crate::hardware::MASTER_CLOCK;

pub struct SquareChannel {
    /// A timer that, when it reaches 64, makes the channel to turn off automatically.
    pub length_timer: u8,

    /// Sets the duty cycle pattern. It's the index of a row of [DUTY_CYCLES].
    pub duty_cycle_pattern: u8,
    /// Sets whether the sound is on or off at this time, depending on the current duty cycle pattern.
    /// It's the index of a column of [DUTY_CYCLES].
    duty_cycle_position: u8,

    /// A counter involved in the wave generation, that decrements at each clock tick.
    /// When it reaches zero, its value is recalculated to a starting value and [Self::duty_cycle_position]
    /// advances.
    frequency_timer: u16,

    /// The raw period of the sound wave. Using a formula, this number can be converted into actual
    /// audible hertz.
    ///
    /// APU doesn't work with _frequencies_ as audio documentation usually does,
    /// but uses time _periods_.
    raw_period: u16,

    /// Control bits.
    controls: Controls,
}

impl SquareChannel {
    const fn tick(&mut self) -> u8 {
        // Calculate the length (in time) of a single duty cycle step.
        // The number of master clock ticks required to represent the current frequency is
        // MASTER_CLOCK / frequency. Since the wave is divided into 8 duty cycle steps, we divide
        // the result by 8.
        let duty_cycle_step_length =
            (MASTER_CLOCK / self.frequency()) as usize / DUTY_CYCLES[0].len();
        0
    }

    /// Returns the raw _period_ of the sound wave.
    ///
    /// Since the raw period is composed of 11 bits, it reaches `2^11 - 1 = 2047`.
    /// This value is way lower than the usual human audible range, which tops at 20 kHz.
    /// A formula is required to covert this raw _period_ value into a proper human frequency.
    ///
    /// This is a convenience method that merges 2 register values.
    pub const fn raw_period(&self) -> u16 {
        self.raw_period
    }

    /// Sets the raw period of the sound wave.
    pub const fn set_raw_period(&mut self, value: u16) {
        self.raw_period = value;
    }

    /// Returns the real frequency, in Hz, computed from the raw period.
    const fn frequency(&self) -> u32 {
        // See https://gbdev.io/pandocs/Audio_Registers.html#ff13--nr13-channel-1-period-low-write-only
        131072 / (2048 - self.raw_period() as u32)
    }

    const fn amplitude(&self) -> u8 {
        DUTY_CYCLES[self.duty_cycle_pattern as usize][self.duty_cycle_position as usize]
    }
}

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
