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

    loop {
        step_cpu(&mut cpu_instance);

        if cpu_instance.is_halted {
            break;
        }
    }
    

    log_step("Program end");

    println!("\nCounters: ");
    println!("sp: {:?}", cpu_instance.sp);
    println!("pc: {:?}", cpu_instance.pc);

    println!("\nRegisters: ");
    println!("a: {:?}", cpu_instance.registers.a);
    println!("b: {:?}", cpu_instance.registers.b);
    println!("c: {:?}", cpu_instance.registers.c);
    println!("d: {:?}", cpu_instance.registers.d);
    println!("e: {:?}", cpu_instance.registers.e);
    println!("f: {:?}", cpu_instance.registers.f);
    println!("h: {:?}", cpu_instance.registers.h);
    println!("l: {:?}", cpu_instance.registers.l);

    println!("bc: {:?}", cpu_instance.registers.get_bc());
    println!("de: {:?}", cpu_instance.registers.get_de());
    println!("hl: {:?}", cpu_instance.registers.get_hl());
}
