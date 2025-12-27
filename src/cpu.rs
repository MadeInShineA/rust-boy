// Instruction structure and names are based on https://gbdev.io/pandocs/CPU_Instruction_Set.html
// Instructions handeling and timings are based on https://gekkio.fi/files/gb-docs/gbctr.pdf
use core::{fmt, panic};
use std::fmt::Debug;

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
            let source_register_bits: u8 = instruction & 0b00000111;
            let (source_register_value, _): (u8, bool) =
                registers.get_r8_register_value(source_register_bits, memory);
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
                registers.a += r8_register_value;
                registers.set_add_flags(old_a_value, registers.a)
            }
            0b10001 => {
                // adc a, r8
                println!("adc a, r8");
                *cycle_counter += 1;
                let carry_value: u8 = if registers.f & Flags::C as u8 != 0 {
                    1
                } else {
                    0
                };
                registers.a += r8_register_value + carry_value;
                registers.set_add_flags(old_a_value, registers.a)
            }
            0b10010 => {
                // sub a, r8
                println!("sub a, r8");
                *cycle_counter += 1;
                registers.a -= r8_register_value;
                registers.set_sub_flags(old_a_value, registers.a)
            }
            0b10011 => {
                // sbc a, r8
                println!("sbc a, r8");
                *cycle_counter += 1;
                let carry_value: u8 = if registers.f & Flags::C as u8 != 0 {
                    1
                } else {
                    0
                };
                registers.a -= r8_register_value - carry_value;
                registers.set_sub_flags(old_a_value, registers.a)
            }
            0b10100 => {
                // and , r8
                println!("and a, r8");
                *cycle_counter += 1;
                registers.a &= r8_register_value;
                registers.set_logical_flags(registers.a, true)
            }
            0b10101 => {
                // xor a, r8
                println!("xor a, r8");
                *cycle_counter += 1;
                registers.a ^= r8_register_value;
                registers.set_logical_flags(registers.a, false)
            }
            0b10110 => {
                // or a, r8
                println!("or a, r8");
                *cycle_counter += 1;
                registers.a |= r8_register_value;
                registers.set_logical_flags(registers.a, false)
            }
            0b10111 => {
                // cp a, r8
                println!("cp a, r8");
                *cycle_counter += 1;
                registers.set_sub_flags(old_a_value, registers.a - r8_register_value)
            }
            _ => panic!("Unknown op_code"),
        }
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
        println!("Type 3 instruction: {instruction:08b}");

        let old_a_value: u8 = registers.a;

        // First sub block operations (a operations)
        match instruction {
            0b11000110 => {
                // add am imm8
                println!("add a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                registers.a += n;
                registers.set_add_flags(old_a_value, registers.a);
            }
            0b11001110 => {
                // adc a, imm8
                println!("adc a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                let carry_value: u8 = if registers.f & Flags::C as u8 != 0 {
                    1
                } else {
                    0
                };
                registers.a += n + carry_value;
                registers.set_add_flags(old_a_value, registers.a);
            }
            0b11010110 => {
                // sub a, imm8
                println!("sub a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                registers.a -= n;
                registers.set_sub_flags(old_a_value, registers.a);
            }
            0b11011110 => {
                // sbc a, imm
                println!("subc a, imm");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                let carry_value: u8 = if registers.f & Flags::C as u8 != 0 {
                    1
                } else {
                    0
                };
                registers.a -= n - carry_value;
                registers.set_sub_flags(old_a_value, registers.a);
            }
            0b11100110 => {
                // and a,imm8
                println!("and a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                registers.a &= n;
                registers.set_logical_flags(registers.a, true);
            }
            0b11101110 => {
                // xor a, imm8
                println!("xor a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                registers.a ^= n;
                registers.set_logical_flags(registers.a, false);
            }
            0b11110110 => {
                // or a, imm8
                println!("or a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                registers.a |= n;
                registers.set_logical_flags(registers.a, false);
            }
            0b11111110 => {
                // cp a, imm8
                println!("cp a, imm8");
                *cycle_counter += 1;
                let n: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                registers.set_sub_flags(old_a_value, registers.a - n);
            }
            _ => {}
        }

        // second sub block operations

        // conditional operations
        let conditional_op_code: u8 = instruction & 0b11100111;
        match conditional_op_code {
            0b1100000 => {
                // ret cond
                println!("ret cond");
                *cycle_counter += 1;
                let condition_bits: u8 = (instruction & 0b00011000) >> 3;

                if registers.should_execute(condition_bits) {
                    *cycle_counter += 1;
                    let nn_lsb: u8 = memory.get_value_at_memory_address(registers.sp);
                    registers.sp += 1;
                    *cycle_counter += 1;

                    let nn_msb: u8 = memory.get_value_at_memory_address(registers.sp);
                    registers.sp += 1;
                    *cycle_counter += 1;

                    registers.pc = ((nn_msb as u16) << 8) | nn_lsb as u16;
                    *cycle_counter += 1
                } else {
                    *cycle_counter += 1
                }
            }
            0b11000010 => {
                // jp cond, imm16
                println!("jp cond, imm16");
                *cycle_counter += 1;
                let condition_bits: u8 = (instruction & 0b00011000) >> 3;

                let nn_lsb: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                let nn_msb: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                if registers.should_execute(condition_bits) {
                    *cycle_counter += 1;
                    registers.pc = ((nn_msb as u16) << 8) | nn_lsb as u16;
                }
            }
            0b11000100 => {
                // call cond, imm16
                println!("call cond, imm16");
                *cycle_counter += 1;
                let condition_bits: u8 = (instruction & 0b00011000) >> 3;

                let nn_lsb: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                let nn_msb: u8 = memory.get_value_at_memory_address(registers.pc);
                registers.pc += 1;
                *cycle_counter += 1;

                if registers.should_execute(condition_bits) {
                    registers.sp -= 1;
                    let pc_msb: u8 = ((registers.pc & 0b1111111100000000) >> 8) as u8;
                    memory.set_value_at_memory_address(registers.sp, pc_msb);
                    registers.sp -= 1;
                    *cycle_counter += 1;

                    let pc_lsb: u8 = (registers.pc & 0b0000000011111111) as u8;
                    memory.set_value_at_memory_address(registers.sp, pc_lsb);
                    *cycle_counter += 1;

                    registers.pc = ((nn_msb as u16) << 8) | nn_lsb as u16;
                    *cycle_counter += 1
                }
            }
            _ => {}
        }

        // target operation
        let target_op_code: u8 = instruction & 0b11000111;
        if target_op_code == 0b11000111 {
            // rst tgt3
            println!("rst tgt3");
            *cycle_counter += 1;

            let target: u8 = (instruction & 0b00111000) >> 3;
            registers.sp -= 1;
            *cycle_counter += 1;

            let pc_msb: u8 = ((registers.pc & 0b1111111100000000) >> 8) as u8;
            memory.set_value_at_memory_address(registers.sp, pc_msb);
            registers.sp -= 1;
            *cycle_counter += 1;

            let pc_lsb: u8 = (registers.pc & 0b0000000011111111) as u8;
            memory.set_value_at_memory_address(registers.sp, pc_lsb);
            registers.pc = (target as u16) * 8;
            *cycle_counter += 1
        }

        // other sub block 2 operations
        // TODO:

        // 3rd block operations (register operations)
        let register_op_code: u8 = instruction & 0b11001111;
        let register_bits: u8 = (instruction & 0b00110000) >> 4;

        match register_op_code {
            0b11000001 => {
                // pop r16stk
                println!("pop r16stk");
                *cycle_counter += 1;

                let lsb: u8 = memory.get_value_at_memory_address(registers.sp);
                registers.sp += 1;
                *cycle_counter += 1;

                let msb: u8 = memory.get_value_at_memory_address(registers.sp);
                registers.sp += 1;
                *cycle_counter += 1;

                let new_register_value: u16 = ((msb as u16) << 8) | lsb as u16;
                registers.set_r16_register_stack_value(register_bits, new_register_value);
            }
            0b11000101 => {
                // push r16stk
                println!("push r16stk");
                *cycle_counter += 1;

                registers.sp -= 1;
                *cycle_counter += 1;

                let register_value: u16 = registers.get_r16_register_stack_value(register_bits);
                let register_value_msb: u8 = ((register_value & 0b1111111100000000) >> 8) as u8;
                let register_value_lsb: u8 = (register_value & 0b0000000011111111) as u8;
                memory.set_value_at_memory_address(registers.sp, register_value_msb);
                *cycle_counter += 1;

                registers.sp -= 1;
                memory.set_value_at_memory_address(registers.sp, register_value_lsb);
                *cycle_counter += 1;
            }
            _ => {}
        }
        // TODO: Prefix and remaining operation sub blocks
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
    fn get_hl_value(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    fn get_r8_register_value(&self, register_bits: u8, memory: &Memory) -> (u8, bool) {
        let was_hl_loaded: bool = register_bits == 0b110;
        let register_value: u8 = match register_bits {
            0b000 => self.b,
            0b001 => self.c,
            0b010 => self.d,
            0b011 => self.e,
            0b100 => self.h,
            0b101 => self.l,
            0b110 => {
                // Loading the value at memory address [hl]
                let memory_address: u16 = self.get_hl_value();
                memory.get_value_at_memory_address(memory_address)
            }
            0b111 => self.a,
            _ => panic!("Invalid register"),
        };
        (register_value, was_hl_loaded)
    }

    fn set_r8_register_value(&mut self, register_bits: u8, value: u8, memory: &mut Memory) {
        match register_bits {
            0b000 => self.b = value,
            0b001 => self.c = value,
            0b010 => self.d = value,
            0b011 => self.e = value,
            0b100 => self.h = value,
            0b101 => self.l = value,
            0b110 => {
                let memory_address: u16 = self.get_hl_value();
                memory.set_value_at_memory_address(memory_address, value);
            }
            0b111 => self.a = value,
            _ => panic!("Invalid register"),
        }
    }

    // TODO: Return value as u16 or direcly msb / lsb ?
    fn get_r16_register_value(&self, register_bits: u8) -> u16 {
        match register_bits {
            0b00 => ((self.b as u16) << 8) | self.c as u16, // BC
            0b01 => ((self.d as u16) << 8) | self.e as u16, // DE
            0b10 => ((self.h as u16) << 8) | self.l as u16, // HL
            0b11 => self.sp,
            _ => panic!("Invalid register"),
        }
    }

    // TODO: Pass value as u16 or direcly msb / lsb ?
    fn set_r16_register_value(&mut self, register_bits: u8, value: u16) {
        let value_lsb: u8 = (value & 0b0000000011111111) as u8;
        let value_msb: u8 = ((value & 0b1111111100000000) >> 8) as u8;
        match register_bits {
            0b00 => {
                // BC
                self.b = value_msb;
                self.c = value_lsb
            }
            0b01 => {
                // DE
                self.d = value_msb;
                self.e = value_lsb
            }
            0b10 => {
                // HL
                self.h = value_msb;
                self.l = value_lsb
            }
            0b11 => {
                // SP
                self.sp = value;
            }
            _ => panic!("Invalid register"),
        }
    }

    fn get_r16_register_stack_value(&self, register_bits: u8) -> u16 {
        match register_bits {
            0b00 => ((self.b as u16) << 8) | self.c as u16, // BC
            0b01 => ((self.d as u16) << 8) | self.e as u16, // DE
            0b10 => ((self.h as u16) << 8) | self.l as u16, // HL
            0b11 => ((self.a as u16) << 8) | self.f as u16, // AF
            _ => panic!("Invalid register"),
        }
    }

    fn set_r16_register_stack_value(&mut self, register_bits: u8, value: u16) {
        let value_lsb: u8 = (value & 0b0000000011111111) as u8;
        let value_msb: u8 = ((value & 0b1111111100000000) >> 8) as u8;
        match register_bits {
            0b00 => {
                // BC
                self.b = value_msb;
                self.c = value_lsb
            }
            0b01 => {
                // DE
                self.d = value_msb;
                self.e = value_lsb
            }
            0b10 => {
                // HL
                self.h = value_msb;
                self.l = value_lsb
            }
            0b11 => {
                // AF
                self.a = value_msb;
                self.f = value_lsb
            }
            _ => panic!("Invalid register"),
        }
    }

    // TODO: getter and setter for r16_register_memory_value

    fn set_add_flags(&mut self, old_value: u8, new_value: u8) {
        self.f = 0b00000000;

        if new_value == 0 {
            self.f |= Flags::Z as u8
        }

        let old_value_lsb: u8 = old_value & 0b00001111;
        let new_value_lsb: u8 = new_value & 0b00001111;

        if new_value_lsb < old_value_lsb {
            self.f |= Flags::H as u8
        }

        if new_value < old_value {
            self.f |= Flags::C as u8
        }
    }

    fn set_sub_flags(&mut self, old_value: u8, new_value: u8) {
        self.f = 0b00000000;

        self.f |= Flags::N as u8;

        if new_value == 0 {
            self.f |= Flags::Z as u8
        }

        let old_value_lsb: u8 = old_value & 0b00001111;
        let new_value_lsb: u8 = new_value & 0b00001111;

        if new_value_lsb > old_value_lsb {
            self.f |= Flags::H as u8
        }

        if new_value > old_value {
            self.f |= Flags::C as u8
        }
    }

    fn set_logical_flags(&mut self, new_value: u8, is_and: bool) {
        self.f = 0b00000000;

        if new_value == 0 {
            self.f |= Flags::Z as u8
        }

        if is_and {
            self.f |= Flags::H as u8
        }
    }

    fn should_execute(&self, condition_bits: u8) -> bool {
        match condition_bits {
            0b00 => (self.f & Flags::Z as u8) == 0, // NZ
            0b01 => (self.f & Flags::Z as u8) != 0, // Z
            0b10 => (self.f & Flags::C as u8) == 0, // NC
            0b11 => (self.f & Flags::C as u8) != 0, // C
            _ => panic!("Invalid condition bits"),
        }
    }
}

enum Flags {
    C = 0b00010000, // Carry flag (bit 7 or 15)
    H = 0b00100000, // Half carry flag (bit 3 or 11)
    N = 0b01000000, // Substraction flag
    Z = 0b10000000, // Zero flag
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
