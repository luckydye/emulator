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
    cpu.jump(jump_condition)

}
