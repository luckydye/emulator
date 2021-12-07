pub mod targets;

use targets::{StackTarget, JumpCondition, ArithmeticTarget, LoadType, LoadByteSource, LoadByteTarget, BitTarget, FlagTarget};

pub mod ADD;
pub mod ADDHL;
pub mod LD;
pub mod JP;
pub mod JR;
pub mod PUSH;
pub mod HALT;
pub mod POP;
pub mod RET;
pub mod CALL;
pub mod XOR;
pub mod BIT;
pub mod NOP;

pub enum Instruction {
    POP(StackTarget),
    PUSH(StackTarget),

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
    BIT(BitTarget, ArithmeticTarget, FlagTarget),
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
    JP(JumpCondition),
    JR(JumpCondition),
    
    RET(JumpCondition),
    CALL(JumpCondition),

    // Loading
    LD(LoadType),

    NOP(),
    HALT(),
}

fn log_inst(n: u8, string: &str) {
    println!("{:#02x}  |  {}", n, string);
}

impl Instruction {

    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            match byte {
                0x7C => {
                    log_inst(byte, "Copy compliment of the the 7th in reg H to the Z Flag");
                    Some(Instruction::BIT(BitTarget::Seven, ArithmeticTarget::H, FlagTarget::Z))
                },
                _ => {
                    panic!("Prefixd instruction not implemented")
                }
            }
        } else {
            match byte {
                0x00 => {
                    log_inst(byte, "Noop");
                    Some(Instruction::NOP())
                },
                0x31 => {
                    log_inst(byte, "Load 2 bytes from direct memory into Stack Pointer");
                    Some(Instruction::LD(LoadType::Word(LoadByteSource::D16, LoadByteTarget::SP)))
                },
                0xAF => {
                    log_inst(byte, "XOR with A and ... A");
                    Some(Instruction::XOR(ArithmeticTarget::A))
                },
                0x09 => {
                    log_inst(byte, "Add BC to HL");
                    Some(Instruction::ADDHL(ArithmeticTarget::BC))
                }, // BC + HL -> HL
                0x20 => {
                    log_inst(byte, "If Zero flag is false jump (signed byte from direct memory) at pc");
                    Some(Instruction::JR(JumpCondition::NotZero))
                },
                0x21 => {
                    log_inst(byte, "Load 2 bytes from direct memory into HL register");
                    Some(Instruction::LD(LoadType::Word(LoadByteSource::D16, LoadByteTarget::HL)))
                },
                0x32 => {
                    log_inst(byte, "Load register a into memory location stored in HL, then decrement HL");
                    Some(Instruction::LD(LoadType::ByteAddressFromRegister(LoadByteSource::A, LoadByteTarget::HLDEC)))
                }, // reg.a -> loc(HL) => dec(HL)
                0x0E => {
                    log_inst(byte, "Load d8 into reg C");
                    Some(Instruction::LD(LoadType::Byte(LoadByteSource::D8, LoadByteTarget::C)))
                },
                0x3E => {
                    log_inst(byte, "Load d8 into reg A");
                    Some(Instruction::LD(LoadType::Byte(LoadByteSource::D8, LoadByteTarget::A)))
                },
                0xE2 => {
                    log_inst(byte, "POP to HL");
                    Some(Instruction::POP(StackTarget::HL))
                },
                0x0C => {
                    log_inst(byte, "Increment C");
                    panic!("Check if incrmenting C works yo")
                    Some(Instruction::INC(ArithmeticTarget::C))
                },
                _ => None
            }
        }
    }

}
