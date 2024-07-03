pub const VRAM_SIZE: usize = 8192;
pub const OAM_RAM_SIZE: usize = 160;

pub struct Gpu {
    pub video_ram: [u8; VRAM_SIZE],
    pub oam_ram: [u8; OAM_RAM_SIZE],
    addressing_mode: AddressingMode,
}

impl Gpu {
    pub fn new_gb() -> Self {
        Self {
            video_ram: [0; VRAM_SIZE],
            oam_ram: [0; OAM_RAM_SIZE],
            addressing_mode: AddressingMode::Unsigned,
        }
    }
}

enum AddressingMode {
    /// This addressing mode uses 0x8000 as the base address, plus
    /// an unsigned offset from it.
    Unsigned,

    /// This addressing mode uses 0x9000 as the base address, plus
    /// a signed offset from it.
    Signed,
}
