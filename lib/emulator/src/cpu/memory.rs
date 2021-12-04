
#[derive(Copy, Clone)]
pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read_word(&self, address: u16) -> u16 {
        let a = self.memory[address as usize];
        let b = self.memory[(address as usize) + 1];
        (a as u16) << 8 | b as u16
    }
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write_byte(mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}