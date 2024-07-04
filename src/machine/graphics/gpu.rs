pub struct Gpu {
    video_ram: VideoRam,
    oam_ram: [u8; OAM_RAM_SIZE],
}

impl Gpu {
    pub fn new_gb() -> Self {
        Self {
            video_ram: VideoRam {
                video_ram: [0; VIDEO_RAM_SIZE],
                addressing_mode: AddressingMode::Unsigned,
            },
            oam_ram: [0; OAM_RAM_SIZE],
        }
    }

    pub fn read_video_ram(&self, addr: u16) -> u8 {
        self.video_ram.read_byte(addr)
    }

    pub fn write_video_ram(&mut self, addr: u16, val: u8) {
        self.video_ram.write_byte(addr, val);
    }

    pub fn read_oam_ram(&self, addr: u16) -> u8 {
        self.oam_ram[addr as usize]
    }

    pub fn write_oam_ram(&mut self, addr: u16, val: u8) {
        self.oam_ram[addr as usize] = val;
    }
}

/// Contains data used by the background and the window.
struct VideoRam {
    video_ram: [u8; VIDEO_RAM_SIZE],
    addressing_mode: AddressingMode,
}

impl VideoRam {
    fn read_byte(&self, idx: u16) -> u8 {
        self.video_ram[self.compute_index(idx)]
    }

    fn write_byte(&mut self, idx: u16, val: u8) {
        self.video_ram[self.compute_index(idx)] = val;
    }

    fn compute_index(&self, addr: u16) -> usize {
        match self.addressing_mode {
            AddressingMode::Unsigned => addr as usize,
            AddressingMode::Signed => 0x1000 + (addr as i16) as usize,
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

const VIDEO_RAM_SIZE: usize = 8192;
const OAM_RAM_SIZE: usize = 160;
