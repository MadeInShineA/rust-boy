mod flat_memory_bus;

pub trait MemoryBus {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, byte: u8);
}

pub use flat_memory_bus::FlatMemoryBus;
