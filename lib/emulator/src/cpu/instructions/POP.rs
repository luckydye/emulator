use super::targets::{StackTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, target: StackTarget) -> u16 {

    let result = cpu.pop();
    match target {
        StackTarget::BC => cpu.registers.set_bc(result),
        StackTarget::HL => cpu.registers.set_hl(result),
        _ => panic!("TODO: support more targets"),
    };
    cpu.pc.wrapping_add(1)

}
