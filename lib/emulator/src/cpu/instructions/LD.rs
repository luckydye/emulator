use super::targets::{LoadType, LoadByteSource, LoadByteTarget};
use crate::cpu::CPU;

pub fn execute(cpu: &mut CPU, load_type: LoadType) -> u16 {

    match load_type {
        LoadType::Byte(source, target) => {
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
                LoadByteTarget::C => cpu.registers.c = source_value,
                LoadByteTarget::HLI => {
                    cpu.bus.write_byte(cpu.registers.get_hl(), source_value)
                }
                _ => panic!("TODO: implement other targets")
            };
            match source {
                LoadByteSource::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        },
        LoadType::Word(source, target) => {
            let source_value = match source {
                LoadByteSource::D16 => cpu.read_next_word(),
                _ => panic!("TODO: implement other sources")
            };
            match target {
                LoadByteTarget::SP => cpu.sp = source_value,
                LoadByteTarget::HL => cpu.registers.set_hl(source_value),
                _ => panic!("TODO: implement other targets")
            };
            match source {
                LoadByteSource::D16 => cpu.pc.wrapping_add(3),
                _ => cpu.pc.wrapping_add(1),
            }
        },
        LoadType::ByteAddressFromRegister(source, target) => {
            let source_value = match source {
                LoadByteSource::A => cpu.registers.a,
                _ => panic!("TODO: implement other sources")
            };
            match target {
                LoadByteTarget::HLDEC => {
                    cpu.bus.write_byte(cpu.registers.get_hl(), source_value);
                    cpu.registers.dec_hl();
                },
                _ => panic!("TODO: implement other targets")
            };
            cpu.pc.wrapping_add(1)
        },
        _ => panic!("TODO: implement other load types")    
    }

}
