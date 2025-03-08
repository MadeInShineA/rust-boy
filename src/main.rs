#![crate_name = "rust_boy"]

mod cpu;
use cpu::Cpu;

mod memory;
use memory::Memory;

fn main() {
    let instructions: Vec<u8> = Vec::from([0b00000001, 0b01000111, 0b10101010, 0b11001110]);

    let memory: Memory = Memory {
        ..Memory::default()
    };

    let mut cpu: Cpu = Cpu {
        memory,
        instructions,
        ..Cpu::default()
    };

    cpu.run();

    println!("{cpu:?}")
}
