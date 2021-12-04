use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU) -> u16 {

    cpu.is_halted = true;
    cpu.pc.wrapping_add(1)

}
