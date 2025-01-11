mod cpu;

use cpu::Cpu;

fn main() {
    let instructions: Vec<u8> = Vec::from([0b00000001, 0b01000111, 0b10101010, 0b11001110]);

    let cpu: Cpu = Cpu {
        instructions,
        ..Cpu::default()
    };

    cpu.run();

    println!("{cpu:?}")
}
