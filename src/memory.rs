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
