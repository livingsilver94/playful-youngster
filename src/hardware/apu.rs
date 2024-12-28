use bitflags::BitFlags8;

const ENVELOPE_DIVIDER: u32 = 8;
const SOUND_LENGHT_DIVIDER: u32 = 2;
const CH1_SWEEP_DIVIDER: u32 = 4;

// https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

pub struct Apu {
    volume: MasterVolume,
    ch1: Channel1,
    ch2: Channel2,
}

impl Apu {
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

struct Channel1 {
    /// Sets the duty cycle pattern. It's the index of a row of [DUTY_CYCLES].
    duty_cycle_pattern: u8,
    /// Sets whether the sound is on or off at this time, depending on the current duty cycle pattern.
    /// It's the index of a column of [DUTY_CYCLES].
    duty_cycle_position: u8,

    /// A timer that, when it reaches 64, makes the channel to turn off automatically.
    length_timer: u8,

    /// Lower 8 bits of the frequency of the sound. The upper 3 bits are contained in [Self::period_upper_ctrl].
    ///
    /// APU doesn't work with _frequencies_ as audio documentation usually does,
    /// but uses time _periods_.
    /// This register acts as a counter that is added +1 for each clock cycle. When it overflows,
    /// the content is reloaded with a formula.
    period_lower: u8,

    /// Upper 3 bits of the frequency of the sound, plus other control bits.
    period_upper_ctrl: PeriodUpperControl,
}

impl Channel1 {
    /// Gets the frequency of the sound.
    /// This is a convenience method that merges 2 register values.
    const fn period(&self) -> u16 {
        (self.period_upper_ctrl.period_upper() as u16) << 8 | self.period_lower as u16
    }

    const fn amplitude(&self) -> u8 {
        DUTY_CYCLES[self.duty_cycle_pattern as usize][self.duty_cycle_position as usize]
    }

    /// Sets the frequency of the sound.
    /// This is a convenience method that merges 2 register values.
    const fn set_period(&mut self, period: u16) {
        self.period_lower = (period & 0xFF) as u8;
        self.period_upper_ctrl
            .set_period_upper((period >> 8) as u8 & 0b111);
    }
}

struct Channel2 {
    /// Sets the duty cycle pattern. It's the index of a row of [DUTY_CYCLES].
    duty_cycle_pattern: u8,
    /// Sets whether the sound is on or off at this time, depending on the current duty cycle pattern.
    /// It's the index of a column of [DUTY_CYCLES].
    duty_cycle_position: u8,

    /// A timer that, when it reaches 64, makes the channel to turn off automatically.
    length_timer: u8,

    /// Lower 8 bits of the frequency of the sound. The upper 3 bits are contained in [Self::period_upper_ctrl].
    ///
    /// APU doesn't work with _frequencies_ as audio documentation usually does,
    /// but uses time _periods_.
    /// This register acts as a counter that is added +1 for each clock cycle. When it overflows,
    /// the content is reloaded with a formula.
    period_lower: u8,

    /// Upper 3 bits of the frequency of the sound, plus other control bits.
    period_upper_ctrl: PeriodUpperControl,
}

impl Channel2 {
    fn step(ticks: u32) {}

    /// Gets the frequency of the sound.
    /// This is a convenience method that merges 2 register values.
    const fn period(&self) -> u16 {
        (self.period_upper_ctrl.period_upper() as u16) << 8 | self.period_lower as u16
    }

    const fn amplitude(&self) -> u8 {
        DUTY_CYCLES[self.duty_cycle_pattern as usize][self.duty_cycle_position as usize]
    }

    /// Sets the frequency of the sound.
    /// This is a convenience method that merges 2 register values.
    const fn set_period(&mut self, period: u16) {
        self.period_lower = (period & 0xFF) as u8;
        self.period_upper_ctrl
            .set_period_upper((period >> 8) as u8 & 0b111);
    }
}

struct PeriodUpperControl(bitflags::BitFlags8);

impl PeriodUpperControl {
    const fn is_triggered(&self) -> bool {
        self.0.get(7)
    }

    const fn is_length_timer_enabled(&self) -> bool {
        self.0.get(6)
    }

    const fn period_upper(&self) -> u8 {
        self.0.get_range(0..=2)
    }

    const fn set_period_upper(&mut self, value: u8) {
        self.0.set_range(0..=2, value);
    }
}

/// Possible duty cycle patterns for the APU.
const DUTY_CYCLES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [0, 0, 0, 0, 0, 0, 1, 1], // 25%
    [0, 0, 0, 0, 1, 1, 1, 1], // 50%
    [1, 1, 1, 1, 1, 1, 0, 0], // 75%
];
