use crate::hardware::{apu::MAX_PERIOD, MASTER_CLOCK};

/// Determines the volume of a channel.
pub struct Volume {
    /// Starting volume. If [Self::sweep] is not zero, it will increase or decrease gradually based on [Self::direction]
    /// and [Self::sweep], otherwise the volume will be constant.
    ///
    /// It is a 4-bit integer, so values above 0xF are not allowed.
    pub volume: u8,
    /// Determines if the volume will increase or decrease gradually.
    pub direction: Direction,
    /// How often the the volume changes, in multiples of the envelope frequency.
    /// For example, a pace value of `2` makes the volume change every `2 * envelope_frequency` Hz.
    /// Zero makes the volume constant.
    pub pace: u8,

    clock_ticks: u32,
    pace_ticks: u8,
}

impl Volume {
    fn tick(&mut self, ticks: u32) -> u8 {
        if self.pace == 0 {
            return self.volume;
        }

        let envelope_ticks = (self.clock_ticks + ticks) / ENVELOPE_CLOCK;
        for _ in 0..envelope_ticks {
            self.pace_ticks += 1;
            if self.pace_ticks == self.pace {
                self.pace_ticks = 0;
                match self.direction {
                    Direction::Increase if self.volume < 0xF => self.volume += 1,
                    Direction::Decrease if self.volume > 0x0 => self.volume -= 1,
                    _ => (),
                }
            }
        }
        self.clock_ticks = (self.clock_ticks + ticks) % ENVELOPE_CLOCK;

        self.volume
    }

    pub fn as_register(&self) -> u8 {
        let direction = if matches!(self.direction, Direction::Increase) {
            1
        } else {
            0
        };
        // INVESTIGATE: self.volume shouldn't change. Does that matter anyway?
        // https://gbdev.io/pandocs/Audio_Registers.html#ff12--nr12-channel-1-volume--envelope
        self.volume << 4 | direction << 3 | self.pace
    }

    pub fn set_from_register(&mut self, register: u8) {
        self.volume = (register & 0b11110000) >> 4;
        self.direction = if register & 0b1000 != 0 {
            Direction::Increase
        } else {
            Direction::Decrease
        };
        self.pace = register & 0b111;
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self {
            volume: (u8::MAX & 0xF) / 2,
            direction: Direction::Decrease,
            pace: Default::default(),

            clock_ticks: Default::default(),
            pace_ticks: Default::default(),
        }
    }
}

pub struct Sweep {
    /// The magnitude of the increase, or decrease, of the period when the sweep effect is triggered.
    /// This magnitude is used in a formula to compute the new period.
    /// If set to zero, the sweep effect is disabled.
    pub step: u8,
    /// Determines if the frequency will increase or decrease gradually.
    pub direction: Direction,
    /// How often the frequency changes, in multiples of the sweep frequency.
    /// For example, a pace value of `2` makes the frequency change every `2 * sweep_frequency` Hz.
    /// If set to zero, pace disables the channel associated to the sweep effect.
    pub pace: u8,

    clock_ticks: u32,
    pace_ticks: u8,
}

impl Sweep {
    pub fn new_disabled() -> Self {
        Self {
            pace: 1,
            ..Default::default()
        }
    }

    /// Computes the new channel period, given the old period and the internal
    /// state of the sweep effect.
    /// An additional boolean value is returned to signal whether the channel is still active;
    /// under certain conditions in fact, the sweep effect disables the channel.
    pub fn tick(&mut self, ticks: u32, mut old_period: u16) -> ChannelSweeped {
        let mut result = ChannelSweeped::Disabled;
        if self.pace == 0 {
            // If pace isn't set, then the sweep effect is disabled and the channel itself will be disabled.
            return result;
        }
        result = ChannelSweeped::NewPeriod(old_period);
        if self.step == 0 {
            // If step isn't set, then the sweep effect is disabled but the channel remains enabled. Basically it's no-op.
            return result;
        }

        let sweep_ticks = (self.clock_ticks + ticks) / SWEEP_CLOCK;
        for _ in 0..sweep_ticks {
            self.pace_ticks += 1;
            if self.pace_ticks == self.pace {
                self.pace_ticks = 0;
                result = self.compute_new_period(old_period);
                if let ChannelSweeped::NewPeriod(period) = result {
                    old_period = period;
                }
            }
        }
        self.clock_ticks = (self.clock_ticks + ticks) % SWEEP_CLOCK;

        result
    }

    pub fn as_register(&self) -> u8 {
        let direction = if matches!(self.direction, Direction::Increase) {
            1
        } else {
            0
        };
        (self.pace << 4) | (direction << 3) | self.step
    }

    pub fn set_from_register(&mut self, register: u8) {
        self.pace = (register & 0b01110000) >> 4;
        self.direction = if register & 0b1000 == 0 {
            Direction::Increase
        } else {
            Direction::Decrease
        };
        self.step = register & 0b111;
    }

    fn compute_new_period(&mut self, old_period: u16) -> ChannelSweeped {
        // See https://gbdev.io/pandocs/Audio_Registers.html#sound-channel-1--pulse-with-period-sweep
        let compute_new_period = |old_period: u16| {
            let period = match self.direction {
                Direction::Decrease => old_period.saturating_sub(old_period >> self.step),
                Direction::Increase => old_period + (old_period >> self.step),
            };
            if period > MAX_PERIOD {
                ChannelSweeped::Disabled
            } else {
                ChannelSweeped::NewPeriod(period)
            }
        };

        let mut new_period = compute_new_period(old_period);
        if let ChannelSweeped::NewPeriod(period) = new_period {
            // If the previous computation didn't disable the channel,
            // the sweep effect recalculates the new period immediately.
            // This time the new computed value is discarded, but the overflow is still checked.
            // See https://gbdev.io/pandocs/Audio_details.html#pulse-channel-with-sweep-ch1
            new_period = compute_new_period(period);
        }

        new_period
    }
}

impl Default for Sweep {
    fn default() -> Self {
        Self {
            step: Default::default(),
            direction: Direction::Increase,
            pace: Default::default(),

            clock_ticks: Default::default(),
            pace_ticks: Default::default(),
        }
    }
}

/// A timer that turns off the channel if enabled.
#[derive(Default)]
pub struct Length<const MAX_TIMER: u16> {
    pub enabled: bool,
    timer: u16,

    clock_ticks: u32,
}

impl<const MAX_TIMER: u16> Length<MAX_TIMER> {
    /// Advances the length timer, if enabled. `true` is returned if the channel is still enabled, `false` otherwise.
    pub fn tick(&mut self, ticks: u32) -> bool {
        if !self.enabled {
            return true;
        }

        let length_ticks = (self.clock_ticks + ticks) / LENGTH_CLOCK;
        self.timer = self.timer.saturating_sub(length_ticks as u16);
        self.clock_ticks = (self.clock_ticks + ticks) % LENGTH_CLOCK;

        self.timer != 0
    }

    pub fn set_timer(&mut self, timer: u8) {
        self.timer = MAX_TIMER - timer as u16;
    }
}

pub enum Direction {
    Decrease,
    Increase,
}

/// Possible outcomes of the sweep effect.
pub enum ChannelSweeped {
    /// A new period is calculated for the channel. This may be identical to the old period.
    NewPeriod(u16),
    /// The channel must be disabled immediately.
    Disabled,
}

const ENVELOPE_CLOCK: u32 = 64; // The envelope effect ticks at 64 Hz.
const SWEEP_CLOCK: u32 = 128; // The sweep effect ticks at 128 Hz.
const LENGTH_CLOCK: u32 = 256; // The length effect ticks at 256 Hz.
