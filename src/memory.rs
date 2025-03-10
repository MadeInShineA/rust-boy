use std::fmt;

pub struct Memory {
    pub memory: [u8; 8192],
}

impl Default for Memory {
    fn default() -> Self {
        Self { memory: [0; 8192] } // Initialize all bytes to 0
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (address, &content) in self.memory.iter().enumerate() {
            write!(f, "{:04X}: {:02X}\t", address, content)?;
        }
        Ok(())
    }
}

impl Memory {
    pub fn get_value_at_memory_address(&self, memory_address: u16) -> u8 {
        self.memory[memory_address as usize]
    }

    pub fn set_value_at_memory_address(&mut self, memory_address: u16, value: u8) {
        self.memory[memory_address as usize] = value
    }
}
