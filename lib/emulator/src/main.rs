use std::vec::Vec;
use std::fs::read;

mod cpu;


fn load_rom(cpu_instance: &mut cpu::CPU, momory: &Vec<u8>) {
    let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];

    let mut i = 0;
    for _ in mem {
        if i < momory.len() {
            mem[i] = momory[i];
            i += 1;
        }
    }

    cpu_instance.load_rom(mem);
}

fn step_cpu(cpu_instance: &mut cpu::CPU) {
    cpu_instance.step();
}

fn log_step(str: &str) {
    println!("{}", str);
}

fn main() {
    let rom_path = "../../roms/boot-rom.gb";
    println!("Rom path: {}", rom_path);
    let data = read(rom_path).unwrap();

    let mut cpu_instance = cpu::CPU::new();

    load_rom(&mut cpu_instance, &data);
    log_step("Boot Rom loaded.");
    
    step_cpu(&mut cpu_instance);
    step_cpu(&mut cpu_instance);
    step_cpu(&mut cpu_instance);
    step_cpu(&mut cpu_instance);

    log_step("Program end");

    println!("sp: {:?}", cpu_instance.sp);
    println!("pc: {:?}", cpu_instance.pc);
}
