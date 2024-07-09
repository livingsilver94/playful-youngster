/// Contains data used by the background and the window.
pub struct Vram {
    pub video_ram: [u8; VRAM_SIZE],
    pub addressing_mode: AddressingMode,
}

impl Vram {
    pub fn new() -> Self {
        Self {
            video_ram: [0; VRAM_SIZE],
            addressing_mode: AddressingMode::Unsigned,
        }
    }

    pub fn read_byte(&self, idx: u16) -> u8 {
        self.video_ram[self.compute_index(idx)]
    }

    pub fn write_byte(&mut self, idx: u16, val: u8) {
        self.video_ram[self.compute_index(idx)] = val;
    }

    fn compute_index(&self, addr: u16) -> usize {
        match self.addressing_mode {
            AddressingMode::Unsigned => addr as usize,
            AddressingMode::Signed => 0x1000 + (addr as i16) as usize,
        }
    }
}

pub enum AddressingMode {
    /// This addressing mode uses 0x8000 as the base address, plus
    /// an unsigned offset from it.
    Unsigned,

    /// This addressing mode uses 0x9000 as the base address, plus
    /// a signed offset from it.
    Signed,
}

const VRAM_SIZE: usize = 8192;
