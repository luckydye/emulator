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
    println!("\nStep.");
    cpu_instance.step();
    print_cpu_state(&cpu_instance);
}

fn log_step(str: &str) {
    println!("{}", str);
}

fn print_cpu_state(cpu: &cpu::CPU) {
    println!("\nCounters: ");
    println!("sp: {:?}", cpu.sp);
    println!("pc: {:?}", cpu.pc);

    println!("\nRegisters: ");
    println!("a: {:b}  b: {:b}", cpu.registers.a, cpu.registers.b);
    println!("c: {:b}  d: {:b}", cpu.registers.c, cpu.registers.d);
    println!("e: {:b}", cpu.registers.e);
    println!("f: {:?}", cpu.registers.f);
    println!("h: {:b}  l: {:b}", cpu.registers.h, cpu.registers.l);

    println!("bc: {:?}", cpu.registers.get_bc());
    println!("de: {:?}", cpu.registers.get_de());
    println!("hl: {:?}", cpu.registers.get_hl());
}

fn main() {
    let rom_path = "../../roms/boot-rom.gb";
    println!("Rom path: {}", rom_path);
    let data = read(rom_path).unwrap();

    let mut cpu_instance = cpu::CPU::new();

    load_rom(&mut cpu_instance, &data);
    log_step("Boot Rom loaded.");
    
    print_cpu_state(&cpu_instance);

    step_cpu(&mut cpu_instance);    // init stack pointer
    step_cpu(&mut cpu_instance);    // a xor with a
    step_cpu(&mut cpu_instance);    // load 2bytes into HL
    step_cpu(&mut cpu_instance);    // reg a to location of hl; hl--
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 
    step_cpu(&mut cpu_instance);    // 


    log_step("Program end");

}
