// Memory map is based on https://gbdev.io/pandocs/Memory_Map.html
use std::fmt;

use crate::memory_bus::MemoryBus;

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
pub struct FlatMemoryBus {
    bytes: [u8; 0x10000],
}

impl fmt::Debug for FlatMemoryBus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row, chunk) in self.bytes.chunks(16).enumerate() {
            write!(f, "{:04X}: ", row * 16)?;
            for byte in chunk {
                write!(f, "{:02X} ", byte)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl MemoryBus for FlatMemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.bytes[address as usize] = value
    }
}

impl FlatMemoryBus {
    pub fn new(bytes: [u8; 0x10000]) -> FlatMemoryBus {
        FlatMemoryBus { bytes }
    }
}
