use super::targets::{JumpTest};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, test: JumpTest) -> u16 {

    let jump_condition = match test {
        JumpTest::NotZero => !cpu.registers.f.zero,
        JumpTest::NotCarry => !cpu.registers.f.carry,
        JumpTest::Zero => cpu.registers.f.zero,
        JumpTest::Carry => cpu.registers.f.carry,
        JumpTest::Always => true,
    };
    cpu.jump(jump_condition)

}
