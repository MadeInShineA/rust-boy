#[derive(Debug)]
pub struct Memory {
    pub memory: [u8; 8192],
}

impl Default for Memory {
    fn default() -> Self {
        Self { memory: [0; 8192] } // Initialize all bytes to 0
    }
}
