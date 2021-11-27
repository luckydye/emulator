pub mod instructions;
pub mod registers;

pub struct CPU {
    pub registers: registers::Registers,
    pub pc: u16
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
        }
    }

    pub fn execute(&mut self, instruction: instructions::Instruction) {
        match instruction {
            instructions::Instruction::ADD(target) => {
                match target {
                    instructions::ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _ => { /* TODO: support more targets */ }
                }
            }
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        // Half Carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        new_value
    }

}

// /cpu
// memory bus
// program counter
// instructions
//  - add
//  - sub
//  - and
//  - or
//  - xor
//  - inc
//  - dec
//  - rr
//  - srl
//  - scf
// registers

// frame capping
// ...
