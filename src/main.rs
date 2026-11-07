#![crate_name = "rust_boy"]

use std::{env, fs::File, io::Read};

mod cpu;
use cpu::Cpu;

mod memory_bus;
use crate::memory_bus::FlatMemoryBus;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    match arguments.len() {
        2 => {
            let file_path = arguments[1].to_string();

            let mut file = File::open(file_path).unwrap();

            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).unwrap();

            let file_size = file_content.len();

            if file_content.is_empty() || file_size > 0x10000 {
                panic!("Bad ROM size: {} bytes", file_content.len());
            }
            let mut loaded_file = [0; 0x10000];
            loaded_file[..file_size].copy_from_slice(&file_content);

            let memory = FlatMemoryBus::new(loaded_file);

            let mut cpu = Cpu::new(memory);

            cpu.run().unwrap();

            println!("{:?}", cpu)
        }
        _ => panic!("usage: rust_boy <rom.gb>"),
    }
}
