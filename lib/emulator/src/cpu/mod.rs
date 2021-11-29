pub mod instructions;
pub mod registers;

pub struct CPU {
    pub registers: registers::Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: MemoryBus,
    pub is_halted: bool,
}

#[derive(Copy, Clone)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_word(&self, address: u16) -> u16 {
        let a = self.memory[address as usize];
        let b = self.memory[(address as usize) + 1];
        (a as u16) << 8 | b as u16
    }

    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write_byte(mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
            pc: 0,
            sp: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
            is_halted: false,
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (msb << 8) | lsb
    }

    fn execute(&mut self, instruction: instructions::Instruction) -> u16 {
        match instruction {
            instructions::Instruction::HALT() => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            instructions::Instruction::CALL(test) => {
                let jump_condition = match test {
                    instructions::JumpTest::NotZero => !self.registers.f.zero,
                    _ => {
                        panic!("TODO: support more conditions")
                    }
                };
                self.call(jump_condition)
            }
            instructions::Instruction::RET(test) => {
                let jump_condition = match test {
                    instructions::JumpTest::NotZero => !self.registers.f.zero,
                    _ => {
                        panic!("TODO: support more conditions")
                    }
                };
                self.return_(jump_condition)
            }
            instructions::Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    instructions::StackTarget::BC => self.registers.set_bc(result),
                    _ => {
                        panic!("TODO: support more targets")
                    }
                };
                self.pc.wrapping_add(1)
            }
            instructions::Instruction::PUSH(target) => {
                let value = match target {
                    instructions::StackTarget::BC => self.registers.get_bc(),
                    _ => {
                        panic!("TODO: support more targets")
                    }
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            instructions::Instruction::LD(load_type) => match load_type {
                instructions::LoadType::Byte(target, source) => {
                    let source_value = match source {
                        instructions::LoadByteSource::A => self.registers.a,
                        instructions::LoadByteSource::D8 => self.read_next_byte(),
                        instructions::LoadByteSource::HLI => {
                            self.bus.read_byte(self.registers.get_hl())
                        }
                        _ => {
                            panic!("TODO: implement other sources")
                        }
                    };
                    match target {
                        instructions::LoadByteTarget::A => self.registers.a = source_value,
                        instructions::LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                        _ => {
                            panic!("TODO: implement other targets")
                        }
                    };
                    match source {
                        instructions::LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
                _ => {
                    panic!("TODO: implement other load types")
                }
            },
            instructions::Instruction::JP(test) => {
                let jump_condition = match test {
                    instructions::JumpTest::NotZero => !self.registers.f.zero,
                    instructions::JumpTest::NotCarry => !self.registers.f.carry,
                    instructions::JumpTest::Zero => self.registers.f.zero,
                    instructions::JumpTest::Carry => self.registers.f.carry,
                    instructions::JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            instructions::Instruction::ADD(target) => {
                match target {
                    instructions::ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        /* TODO: support more targets */
                        self.pc
                    }
                }
            }
            _ => {
                /* TODO: support more instructions */
                self.pc
            }
        }
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn read_next_word(&self) -> u16 {
        self.bus.read_word(self.pc + 1)
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) =
            instructions::Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!(
                "0x{}{:x}",
                if prefixed { "cb" } else { "" },
                instruction_byte
            );
            panic!("Unkown instruction found for: {}", description)
        };
        self.pc = next_pc;
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
// registers
