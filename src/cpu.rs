// Instruction structure and names are based on https://gbdev.io/pandocs/CPU_Instruction_Set.html
// Instructions handeling and timings are based on https://gekkio.fi/files/gb-docs/gbctr.pdf
use core::{fmt, panic};
use std::{fmt::Debug, ops::BitOrAssign};

use crate::memory::Memory;

#[derive(Debug, Default)]
pub struct Type0InstructionHandler {}

impl Type0InstructionHandler {
    fn handle_instruction(
        &self,
        instruction: u8,
        cycle_counter: &mut u64,
        registers: &mut Registers,
    ) {
        println!("Type 0 instruction: {instruction:08b}");
    }
}

#[derive(Debug, Default)]
pub struct Type1InstructionHandler {}

impl Type1InstructionHandler {
    fn handle_instruction(
        &self,
        instruction: u8,
        cycle_counter: &mut u64,
        registers: &mut Registers,
        memory: &mut Memory,
        is_halting: &mut bool,
    ) {
        println!("Type 1 instruction: {instruction:08b}");

        // if destination_register and source_register == [hl]
        if instruction == 0b01110110 {
            // halt
            println!("halt");
            *cycle_counter += 1;
            *is_halting = true
        } else {
            // ld r8, r8
            println!("ld r8, r8");
            *cycle_counter += 1;
            let destination_register: u8 = (instruction & 0b00111000) >> 3;
            let source_register: u8 = instruction & 0b00000111;
            let (source_register_value, _): (u8, bool) =
                registers.get_r8_register_value(source_register, memory);
            registers.set_r8_register_value(destination_register, source_register_value, memory);
        }
    }
}

#[derive(Debug, Default)]
pub struct Type2InstructionHandler {}

impl Type2InstructionHandler {
    fn handle_instruction(
        &self,
        instruction: u8,
        cycle_counter: &mut u64,
        registers: &mut Registers,
        memory: &Memory,
    ) {
        println!("Type 2 instruction: {instruction:08b}");
        let op_code: u8 = (instruction & 0b11111000) >> 3;
        let operand: u8 = instruction & 0b00000111;

        println!("Op code: {op_code:05b} Operand: {operand:03b}");
        let (r8_register_value, was_hl_loaded) = registers.get_r8_register_value(operand, memory);

        if was_hl_loaded {
            *cycle_counter += 1
        }

        let old_a_value: u8 = registers.a;

        match op_code {
            0b10000 => {
                // add a, r8
                println!("add a, r8");
                *cycle_counter += 1;
                registers.a += r8_register_value
            }
            0b10001 => {
                // adc a, r8
                println!("adc a, r8");
                *cycle_counter += 1;
                registers.a += r8_register_value + Flags::C as u8;
            }
            0b10010 => {
                // sub a, r8
                println!("sub a, r8");
                *cycle_counter += 1;
                registers.a -= r8_register_value
            }
            0b10011 => {
                // sbc a, r8
                println!("sbc a, r8");
                *cycle_counter += 1;
                registers.a -= r8_register_value - Flags::C as u8
            }
            0b10100 => {
                // and , r8
                println!("and a, r8");
                *cycle_counter += 1;
                registers.a &= r8_register_value
            }
            0b10101 => {
                // xor a, r8
                println!("xor a, r8");
                *cycle_counter += 1;
                registers.a ^= r8_register_value
            }
            0b10110 => {
                // or a, r8
                println!("or a, r8");
                *cycle_counter += 1;
                registers.a |= r8_register_value
            }
            0b10111 => {
                // cp a, r8
                println!("cp a, r8");
                *cycle_counter += 1;
                registers.set_flags_for_r8_opperation(old_a_value, old_a_value - r8_register_value)
            }
            _ => panic!("Unknown op_code"),
        }

        let new_a_value: u8 = registers.a;

        registers.set_flags_for_r8_opperation(old_a_value, new_a_value);
    }
}

#[derive(Debug, Default)]
pub struct Type3InstructionHandler {}

impl Type3InstructionHandler {
    fn handle_instruction(
        &self,
        instruction: u8,
        cycle_counter: &mut u64,
        registers: &mut Registers,
        memory: &mut Memory,
    ) {
        println!("Type 3 instruction: {instruction:09b}");

        let old_a_value = registers.a;

        // First block match (a operations)
        match instruction {
            0b11000110 => {
                // add am imm8
                println!("add a, imm8");
                registers.a += instruction;
                *cycle_counter += 2
            }
            0b11001110 => {
                // adc a, imm8
                println!("adc a, imm8");
                registers.a += instruction + Flags::C as u8;
                *cycle_counter += 2
            }
            0b11010110 => {
                // sub a, imm8
                println!("sub a, imm8");
                registers.a -= instruction;
                *cycle_counter += 2
            }
            0b1001110 => {
                // sbc a, imm
                println!("subc a, imm");
                registers.a -= instruction - Flags::C as u8;
                *cycle_counter += 2
            }
            0b11100110 => {
                // and a,imm8
                println!("and a, imm8");
                registers.a &= instruction;
                *cycle_counter += 2
            }
            0b11101110 => {
                // xor a, imm8
                println!("xor a, imm8");
                registers.a ^= instruction;
                *cycle_counter += 2
            }
            0b11110110 => {
                // or a, imm8
                println!("or a, imm8");
                registers.a |= instruction;
                *cycle_counter += 2
            }
            0b1111110 => {
                // cp a, imm8
                println!("cp a, imm8");
                registers.set_flags_for_r8_opperation(old_a_value, old_a_value - instruction);
                *cycle_counter += 2
            }
            _ => {}
        }

        // second block match

        // conditional operations
        let conditional_op_code = instruction & 0b11100111;
        match conditional_op_code {
            0b1100000 => {
                // ret cond
                println!("ret cond");
                *cycle_counter += 1;
                let condition: u8 = (instruction & 0b00011000) >> 3;

                if condition != Flags::Z as u8 {
                    *cycle_counter += 1;
                    let last_8_bits: u8 = memory.get_value_at_memory_address(registers.sp);
                    *cycle_counter += 1;
                    registers.sp += 1;
                    let first_8_bits: u8 = memory.get_value_at_memory_address(registers.sp);
                    *cycle_counter += 1;
                    registers.sp += 1;
                    registers.pc = ((first_8_bits as u16) << 8) | last_8_bits as u16;
                    *cycle_counter += 1
                } else {
                    *cycle_counter += 1
                }
            }
            0b11000010 => {
                // jp cond, imm16
                println!("jp cond, imm16");
                *cycle_counter += 1;
                let condition: u8 = (instruction & 0b00011000) >> 3;

                let last_8_bits: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;
                let first_8_bits: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                if condition != Flags::Z as u8 {
                    *cycle_counter += 1;
                    registers.pc = ((first_8_bits as u16) << 8) | last_8_bits as u16;
                }
            }
            0b11000100 => {
                // call cond, imm16
                println!("call cond, imm16");
                *cycle_counter += 1;
                let condition: u8 = (instruction & 0b00011000) >> 3;

                let last_8_bits: u8 = memory.get_value_at_memory_address(registers.pc);
                *cycle_counter += 1;
                registers.pc += 1;
                let first_8_bits: u8 = memory.get_value_at_memory_address(registers.pc);
                *cycle_counter += 1;
                registers.pc += 1;

                if condition != Flags::Z as u8 {
                    registers.sp = registers.pc;
                    *cycle_counter += 2;

                    registers.pc = ((first_8_bits as u16) << 8) | last_8_bits as u16;
                    *cycle_counter += 1
                }
            }
            _ => {}
        }

        // target operations
        let target_op_code: u8 = instruction & 0b11000111;
        if target_op_code == 0b11000111 {
            // rst tgt3
            println!("rst tgt3");
            *cycle_counter += 1;

            let target: u8 = (target_op_code & 0b00111000) >> 3;
            registers.sp -= 1;

            let pc_msb: u8 = ((registers.pc & 0b1111111100000000) >> 8) as u8;
            memory.set_value_at_memory_address(registers.sp, pc_msb);
            *cycle_counter += 1;
            registers.sp -= 1;
            let pc_lsb: u8 = (registers.pc & 0b0000000011111111) as u8;
            *cycle_counter += 1;
            memory.set_value_at_memory_address(registers.pc, pc_lsb);
            registers.pc = target as u16
        }
    }
}

#[derive(Default)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AF:  {:#04X} {:#04X}\nBC:  {:#04X} {:#04X}\nDE:  {:#04X} {:#04X}\nHL:  {:#04X} {:#04X}\nSP:  {:#06X}\nPC:  {:#06X}",
            self.a, self.f, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc
        )
    }
}

impl Registers {
    fn get_r8_register_value(&self, register: u8, memory: &Memory) -> (u8, bool) {
        let was_hl_loaded: bool = register == 6;
        let register_value: u8 = match register {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => {
                // Loading the value at memory address [hl]
                let memory_address: u16 = self.get_hl_value();
                memory.get_value_at_memory_address(memory_address)
            }
            7 => self.a,
            _ => panic!("Invalid register"),
        };
        (register_value, was_hl_loaded)
    }

    fn set_r8_register_value(&mut self, register: u8, value: u8, memory: &mut Memory) {
        match register {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => {
                let memory_address: u16 = self.get_hl_value();
                memory.set_value_at_memory_address(memory_address, value);
            }
            7 => self.a = value,
            _ => panic!("Invalid register"),
        }
    }

    fn get_hl_value(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    fn set_flags_for_r8_opperation(&mut self, old_value: u8, new_value: u8) {
        // Reset the Flags
        self.f = 0b00000000;

        let bit_3_from_old_value: u8 = (old_value >> 3) & 0b00000001;
        let bit_7_from_old_value: u8 = (old_value >> 7) & 0b00000001;

        let bit_3_from_new_value: u8 = (new_value >> 3) & 0b00000001;
        let bit_7_from_new_value: u8 = (new_value >> 7) & 0b00000001;

        if new_value == 0 {
            self.f |= Flags::Z as u8
        }

        if new_value < old_value {
            self.f |= Flags::N as u8
        }

        // If bit 3 carry
        if bit_3_from_old_value == 1 && bit_3_from_new_value == 0 {
            self.f |= Flags::H as u8
        }

        // If bit 7 carry
        if bit_7_from_old_value == 1 && bit_7_from_new_value == 0 {
            self.f |= Flags::C as u8
        }
    }
}

enum Flags {
    C = 0b0001000, // Carry flag (bit 7 or 15)
    H = 0b0010000, // Half carry flag (bit 3 or 11)
    N = 0b0100000, // Substraction flag
    Z = 0b1000000, // Zero flag
}

#[derive(Default)]
pub struct Cpu {
    pub is_halting: bool,
    pub cycle_counter: u64,
    pub instructions: Vec<u8>,
    pub registers: Registers,
    pub memory: Memory,
    pub type0_instruction_handler: Type0InstructionHandler,
    pub type1_instruction_handler: Type1InstructionHandler,
    pub type2_instruction_handler: Type2InstructionHandler,
    pub type3_instruction_handler: Type3InstructionHandler,
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "========== CPU ==========\n===== Instructions =====\n{:?}\n===== Registers =====\n{:?}\n===== Memory =====\n{:?}",
            self.instructions, self.registers, self.memory
        )
    }
}

impl Cpu {
    pub fn handle_instruction(&mut self, instruction: u8) {
        let op_type: u8 = instruction >> 6;
        match op_type {
            0b00 => self.type0_instruction_handler.handle_instruction(
                instruction,
                &mut self.cycle_counter,
                &mut self.registers,
            ),

            0b01 => self.type1_instruction_handler.handle_instruction(
                instruction,
                &mut self.cycle_counter,
                &mut self.registers,
                &mut self.memory,
                &mut self.is_halting,
            ),

            0b10 => self.type2_instruction_handler.handle_instruction(
                instruction,
                &mut self.cycle_counter,
                &mut self.registers,
                &self.memory,
            ),

            0b11 => self.type3_instruction_handler.handle_instruction(
                instruction,
                &mut self.cycle_counter,
                &mut self.registers,
                &mut self.memory,
            ),
            _ => panic!("Unknown operation type"),
        }
    }

    pub fn run(&mut self) {
        while self.registers.pc < self.instructions.len() as u16 {
            if !self.is_halting {
                let current_instruction: u8 = self.instructions[self.registers.pc as usize];
                self.registers.pc += 1;
                self.handle_instruction(current_instruction);
            }
        }
    }
}
