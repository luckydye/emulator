use super::targets::{JumpTest};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, test: JumpTest) -> u16 {

    let jump_condition = match test {
        JumpTest::NotZero => !cpu.registers.f.zero,
        _ => {
            panic!("TODO: support more conditions")
        }
    };
    cpu.return_(jump_condition)

}
