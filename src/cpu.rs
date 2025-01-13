#[derive(Debug, Default)]
pub struct Type0InstructionHandler {}

impl Type0InstructionHandler {
    fn handle_instruction(&self, registers: &mut Registers, instruction: u8) {
        registers.a = 12;
        println!("Type 0 instruction: {instruction:08b}");
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
        println!("Type 2 instruction: {instruction:08b}");
        let op_code: u8 = (instruction >> 3) & 0b00011111;
        let operand: u8 = instruction & 0b00000111;

        println!("Op code: {op_code:05b} Operand: {operand:03b}");
        if let RegisterValue::U8(r8_register_value) = registers.get_r8_register(operand) {
            // TODO Add the correct Flags
            match op_code {
                0b10000 => {
                    println!("add");
                    registers.a += r8_register_value
                    // TODO Add [hl] handling
                }
                0b10001 => println!("adc"),
                0b10010 => {
                    println!("sub");
                    registers.a -= r8_register_value;
                    // TODO Add [hl handling]
                }
                0b10011 => println!("sbc"),
                0b10100 => {
                    println!("and");
                    registers.a &= r8_register_value;
                    // TODO Add [hl] handling
                }
                0b10101 => {
                    println!("xor");
                    registers.a ^= r8_register_value;
                    // TODO Add [hl] handling
                }
                0b10110 => {
                    println!("or");
                    registers.a |= r8_register_value;
                    // TODO Add [hl] handling
                }
                0b10111 => println!("cp"),
                _ => panic!("Unknown op_code"),
            }
        } else {
            panic!("Unexpected RegisterValue: Expected RegisterValue::U8")
        }
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

enum RegisterValue {
    U8(u8),
    U16(u16),
}

impl Registers {
    fn get_r8_register(&self, register: u8) -> RegisterValue {
        match register {
            0 => RegisterValue::U8(self.b),
            1 => RegisterValue::U8(self.c),
            2 => RegisterValue::U8(self.d),
            3 => RegisterValue::U8(self.e),
            4 => RegisterValue::U8(self.h),
            5 => RegisterValue::U8(self.l),
            // What's [hl] ?
            // 6 => self.get_value_at_hl(),
            7 => RegisterValue::U8(self.a),
            _ => panic!("Invalid register"),
        }
    }

    fn get_value_at_hl(&self) -> RegisterValue {
        let res: u16 = ((self.h as u16) << 8) | (self.l as u16);
        RegisterValue::U16(res)
    }
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
        let op_type: u8 = instruction >> 6;
        match op_type {
            0b00 => self
                .type0_instruction_handler
                .handle_instruction(&mut self.registers, instruction),

            0b01 => self
                .type1_instruction_handler
                .handle_instruction(&mut self.registers, instruction),

            0b10 => self
                .type2_instruction_handler
                .handle_instruction(&mut self.registers, instruction),

            0b11 => self
                .type3_instruction_handler
                .handle_instruction(&mut self.registers, instruction),
            _ => panic!("Unknown operation type"),
        }
    }

    pub fn run(&mut self) {
        while self.registers.pc < self.instructions.len() as u16 {
            self.handle_instruction(self.instructions[self.registers.pc as usize]);
            self.registers.pc += 1
        }
    }
}
