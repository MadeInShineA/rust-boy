#[derive(Debug, Default)]
pub struct Type0InstructionHandler {}

impl Type0InstructionHandler {
    fn handle_instruction(&self, registers: &mut Registers, instruction: u8) {
        registers.a = 12;
        println!("Type 0 instruction: {instruction:08b}")
    }
}

#[derive(Debug, Default)]
pub struct Type1InstructionHandler {}

impl Type1InstructionHandler {
    fn handle_instruction(&self, registers: &mut Registers, instruction: u8) {
        registers.b = 12;
        println!("Type 1 instruction: {instruction:08b}")
    }
}

#[derive(Debug, Default)]
pub struct Type2InstructionHandler {}

impl Type2InstructionHandler {
    fn handle_instruction(&self, registers: &mut Registers, instruction: u8) {
        registers.c = 12;
        println!("Type 2 instruction: {instruction:08b}")
    }
}

#[derive(Debug, Default)]
pub struct Type3InstructionHandler {}

impl Type3InstructionHandler {
    fn handle_instruction(&self, registers: &mut Registers, instruction: u8) {
        registers.d = 12;
        println!("Type 3 instruction: {instruction:08b}")
    }
}

#[derive(Debug, Default)]
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

enum Flags {
    C = 0b0001000,
    H = 0b0010000,
    N = 0b0100000,
    Z = 0b1000000,
}

#[derive(Debug, Default)]
pub struct Cpu {
    pub instructions: Vec<u8>,
    pub registers: Registers,
    pub type0_instruction_handler: Type0InstructionHandler,
    pub type1_instruction_handler: Type1InstructionHandler,
    pub type2_instruction_handler: Type2InstructionHandler,
    pub type3_instruction_handler: Type3InstructionHandler,
}

impl Cpu {
    pub fn handle_instruction(&mut self, instruction: u8) {
        match instruction {
            0b00000000..=0b00111111 => self
                .type0_instruction_handler
                .handle_instruction(&mut self.registers, instruction),

            0b01000000..=0b01111111 => self
                .type1_instruction_handler
                .handle_instruction(&mut self.registers, instruction),

            0b10000000..=0b10111111 => self
                .type2_instruction_handler
                .handle_instruction(&mut self.registers, instruction),

            0b11000000..=0b11111111 => self
                .type3_instruction_handler
                .handle_instruction(&mut self.registers, instruction),
        }
    }

    pub fn run(&mut self) {
        for instruction in self.instructions.clone() {
            self.handle_instruction(instruction);
        }
    }
}
