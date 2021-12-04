use std::vec::Vec;
use std::fs::read;

mod cpu;

enum EmulationError {
    RomLoadingError,
    CPUStepError,
    LogError,
}

fn load_rom(cpu_instance: &mut cpu::CPU, momory: &Vec<u8>) -> Result<(), EmulationError> {
    let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];

    let mut i = 0;
    for _ in mem {
        if i < momory.len() {
            mem[i] = momory[i];
            i += 1;
        }
    }

    cpu_instance.load_rom(mem);

    Ok(())
}

fn step_cpu(cpu_instance: &mut cpu::CPU) -> Result<(), EmulationError> {
    cpu_instance.step();
    Err(EmulationError::CPUStepError)
}

fn log_step(str: &str) -> Result<(), EmulationError> {
    println!("{}", str);
    Ok(())
}

fn main() {
    let rom_path = "../../roms/boot-rom.gb";
    println!("Rom path: {}", rom_path);
    let data = read(rom_path).unwrap();

    let mut cpu_instance = cpu::CPU::new();

    let mut do_steps = || -> Result<(), EmulationError> {
        load_rom(&mut cpu_instance, &data)?;
        log_step("Boot Rom loaded.")?;
        step_cpu(&mut cpu_instance)?;
        log_step("Program end")?;
        Ok(())
    };

    if let Err(_err) = do_steps() {
        println!("Failed to perform necessary steps");
    }

    println!("{:?}", cpu_instance.registers.a);
}
