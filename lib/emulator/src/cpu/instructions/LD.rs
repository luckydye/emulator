use super::targets::{LoadType, LoadByteSource, LoadByteTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, load_type: LoadType) -> u16 {

    match load_type {
        LoadType::Byte(target, source) => {
            let source_value = match source {
                LoadByteSource::A => cpu.registers.a,
                LoadByteSource::D8 => cpu.read_next_byte(),
                LoadByteSource::HLI => {
                    cpu.bus.read_byte(cpu.registers.get_hl())
                }
                _ => panic!("TODO: implement other sources")
            };
            match target {
                LoadByteTarget::A => cpu.registers.a = source_value,
                LoadByteTarget::HLI => {
                    cpu.bus.write_byte(cpu.registers.get_hl(), source_value)
                }
                _ => panic!("TODO: implement other targets")
            };
            match source {
                LoadByteSource::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }
        _ => panic!("TODO: implement other load types")    
    }

}
