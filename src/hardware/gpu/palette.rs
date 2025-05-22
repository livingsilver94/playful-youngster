#[derive(Clone, Copy, Default)]
pub struct PaletteIds(bitflags::BitFlags8);

impl PaletteIds {
    pub fn id3(&self) -> Palette {
        self.palette_from_bits(7, 6)
    }

    pub fn id2(&self) -> Palette {
        self.palette_from_bits(5, 4)
    }

    pub fn id1(&self) -> Palette {
        self.palette_from_bits(3, 2)
    }

    pub fn id0(&self) -> Palette {
        self.palette_from_bits(1, 0)
    }

    fn palette_from_bits(&self, bit1: usize, bit2: usize) -> Palette {
        From::from(((self.0.get(bit1) as u8) << 1) | (self.0.get(bit2) as u8))
    }
}

#[repr(u8)]
pub enum Palette {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        match value & 0b00000011 {
            0 => Self::White,
            1 => Self::LightGray,
            2 => Self::DarkGray,
            _ => Self::Black,
        }
    }
}

impl From<u8> for PaletteIds {
    fn from(value: u8) -> Self {
        Self(From::from(value))
    }
}

impl From<PaletteIds> for u8 {
    fn from(value: PaletteIds) -> Self {
        From::from(value.0)
    }
}
