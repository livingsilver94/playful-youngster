pub struct Memory {
    ram: [u8; 0xFFFF + 1],
}

impl Memory {
    pub fn at(&self, addr: u16) -> u8 {
        *self.ram.get(addr as usize).unwrap()
    }

    pub fn at_mut(&mut self, addr: u16) -> &mut u8 {
        self.ram.get_mut(addr as usize).unwrap()
    }
}
