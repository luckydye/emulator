use super::targets::{StackTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, target: StackTarget) -> u16 {

    let value = match target {
        StackTarget::BC => cpu.registers.get_bc(),
        _ => panic!("TODO: support more targets")
    };
    cpu.push(value);
    cpu.pc.wrapping_add(1)

}
