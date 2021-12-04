use super::targets::{ArithmeticTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, target: ArithmeticTarget) -> u16 {

    match target {
        ArithmeticTarget::C => {
            let value = cpu.registers.c;
            let new_value = cpu.add(value);
            cpu.registers.a = new_value;
            cpu.pc.wrapping_add(1)
        }
        _ => cpu.pc
    }

}
