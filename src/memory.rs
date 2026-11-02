// Memory map is based on https://gbdev.io/pandocs/Memory_Map.html
use std::fmt;

// 0000-3FFF   ROM bank 00          Cartridge, fixed bank
// 4000-7FFF   ROM bank 01-NN       Cartridge, switchable bank (via MBC)
// 8000-9FFF   VRAM                 Video RAM (tiles and tile maps)
// A000-BFFF   External RAM         Cartridge RAM (saves), if any
// C000-CFFF   WRAM bank 0          Work RAM
// D000-DFFF   WRAM bank 1          Work RAM
// E000-FDFF   Echo RAM             Mirror of C000-DDFF, should not be used
// FE00-FE9F   OAM                  Sprite attribute table
// FEA0-FEFF   Not usable
// FF00-FF7F   I/O registers        Joypad, serial, timer, sound, LCD...
// FF80-FFFE   HRAM                 High RAM (stack usually lives here)
// FFFF        IE                   Interrupt enable register
pub struct Memory {
    pub memory: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            memory: [0; 0x10000],
        } // Initialize all bytes to 0
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row, chunk) in self.memory.chunks(16).enumerate() {
            write!(f, "{:04X}: ", row * 16)?;
            for byte in chunk {
                write!(f, "{:02X} ", byte)?;
            }
            writeln!(f)?;
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
