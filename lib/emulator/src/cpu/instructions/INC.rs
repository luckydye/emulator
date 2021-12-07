use super::targets::{ArithmeticTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, target: ArithmeticTarget) -> u16 {

    match target {
        ArithmeticTarget::C => {
            cpu.registers.c = cpu.registers.c + 1;
            cpu.pc.wrapping_add(1)
        }
        _ => cpu.pc
    }

}
