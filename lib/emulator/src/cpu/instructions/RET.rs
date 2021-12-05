use super::targets::{JumpCondition};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, test: JumpCondition) -> u16 {

    let jump_condition = match test {
        JumpCondition::NotZero => !cpu.registers.f.zero,
        _ => {
            panic!("TODO: support more conditions")
        }
    };
    cpu.return_(jump_condition)

}
