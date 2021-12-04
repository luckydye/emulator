use super::targets::{ArithmeticTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, target: ArithmeticTarget) -> u16 {

    match target {
        ArithmeticTarget::A => {
            let value = cpu.registers.a;
            cpu.registers.a = value ^ cpu.registers.a;
            cpu.pc.wrapping_add(1)
        }
        _ => cpu.pc
    }

}
