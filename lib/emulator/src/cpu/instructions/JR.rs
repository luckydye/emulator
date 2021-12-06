use super::targets::{JumpCondition};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, test: JumpCondition) -> u16 {

    let jump_condition = match test {
        JumpCondition::NotZero => !cpu.registers.f.zero,
        JumpCondition::NotCarry => !cpu.registers.f.carry,
        JumpCondition::Zero => cpu.registers.f.zero,
        JumpCondition::Carry => cpu.registers.f.carry,
        JumpCondition::Always => true,
    };

    if jump_condition {
        let b = cpu.read_next_byte();

        let sign_mask = 0x80;
        let v = b & !sign_mask;

        if b & sign_mask > 0 {
            // positve
            cpu.pc.wrapping_add(v as u16)
        } else {
            // negative
            cpu.pc.wrapping_sub(v as u16)
        }

    } else {
        cpu.pc.wrapping_add(2)
    }
}
