use super::targets::{BitTarget, ArithmeticTarget, FlagTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, bit_target: BitTarget, reg_target: ArithmeticTarget, flag_target: FlagTarget) -> u16 {

    let reg_value = match reg_target {
        ArithmeticTarget::H => {
            cpu.registers.h
        },
        _ => panic!("Implement more BIT targets")
    };

    match bit_target {
        BitTarget::Seven => {
            match flag_target {
                FlagTarget::Z => {
                    cpu.registers.f.zero = (0x01 & reg_value) == 0;
                }
            }
        },
        _ => panic!("Implement more BIT targets")
    }

    cpu.pc.wrapping_add(2)
}
