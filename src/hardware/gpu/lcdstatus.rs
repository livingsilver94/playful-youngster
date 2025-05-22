#[derive(Clone, Copy, Default)]
pub struct LcdStatus(bitflags::BitFlags8);

impl LcdStatus {
    /// Returns whether the STAT interrupt is fired when
    /// the LYC register is equal to the LY register.
    pub fn interrupt_if_lyc_equals_ly(&self) -> bool {
        self.0.get(6)
    }

    /// Returns whether the STAT interrupt is fired when
    /// the PPU is in mode 2.
    pub fn interrupt_if_mode2(&self) -> bool {
        self.0.get(5)
    }

    /// Returns whether the STAT interrupt is fired when
    /// the PPU is in mode 1.
    pub fn interrupt_if_mode1(&self) -> bool {
        self.0.get(4)
    }

    /// Returns whether the STAT interrupt is fired when
    /// the PPU is in mode 0.
    pub fn interrupt_if_mode0(&self) -> bool {
        self.0.get(3)
    }

    /// Returns whether the LYC register is currently
    /// equal to the LY register.
    pub fn lyc_equals_ly(&self) -> bool {
        self.0.get(2)
    }

    /// Returns the current PPU mode.
    pub fn ppu_mode(&self) -> PpuMode {
        let value = ((self.0.get(1) as u8) << 1) | self.0.get(0) as u8;
        match value {
            0 => PpuMode::Mode0,
            1 => PpuMode::Mode1,
            2 => PpuMode::Mode2,
            3 => PpuMode::Mode3,
            _ => unreachable!(),
        }
    }
}

impl From<LcdStatus> for u8 {
    fn from(value: LcdStatus) -> Self {
        From::from(value.0)
    }
}

impl From<u8> for LcdStatus {
    fn from(value: u8) -> Self {
        Self(From::from(value))
    }
}

/// Possible modes the PPU may be in.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum PpuMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}
