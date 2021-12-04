pub mod targets;

use targets::{StackTarget, JumpTest, ArithmeticTarget, LoadType, LoadByteSource, LoadByteTarget};

pub mod ADD;
pub mod ADDHL;
pub mod LD;
pub mod JP;
pub mod PUSH;
pub mod HALT;
pub mod POP;
pub mod RET;
pub mod CALL;
pub mod XOR;

pub enum Instruction {
    POP(StackTarget),
    PUSH(StackTarget),
    
    RET(JumpTest),
    CALL(JumpTest),

    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    CCF(ArithmeticTarget),
    SCF(ArithmeticTarget),
    RRA(ArithmeticTarget),
    RLA(ArithmeticTarget),
    RRCA(ArithmeticTarget),
    RRLA(ArithmeticTarget),
    CPL(ArithmeticTarget),
    BIT(ArithmeticTarget),
    RESET(ArithmeticTarget),
    SET(ArithmeticTarget),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
    // Jumps
    JP(JumpTest),

    // Loading
    LD(LoadType),

    NOP(),
    HALT(),
}

impl Instruction {

    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            match byte {
                // 0x00 => Some(Instruction::RLC(PrefixTarget::B)),
                _ => None
            }
        } else {
            match byte {
                0x31 => Some(Instruction::LD(LoadType::Word(LoadByteSource::D16, LoadByteTarget::SP))),
                0xAF => Some(Instruction::XOR(ArithmeticTarget::A)),
                0x09 => Some(Instruction::ADDHL(ArithmeticTarget::BC)), // BC + HL -> HL
                0x21 => Some(Instruction::LD(LoadType::Word(LoadByteSource::D16, LoadByteTarget::HL))),
                0x32 => Some(),
                _ => None
            }
        }
    }

}
