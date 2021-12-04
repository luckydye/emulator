use wasm_bindgen::prelude::*;

mod cpu;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn log(s: &[u8]);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn set_input_state(input_state: &[u8]) {
    
}

enum EmulationError {
    RomLoadingError,
    CPUStepError,
    LogError,
}

fn load_rom(cpu_instance: &mut cpu::CPU, momory: &[u8]) -> Result<(), EmulationError> {
    let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];

    let mut i = 0;
    for _ in mem {
        mem[i] = momory[i];
        i += 1;
    }

    log(&mem);

    cpu_instance.load_rom(mem);
    Ok(())
}

fn step_cpu(cpu_instance: &mut cpu::CPU) -> Result<(), EmulationError> {
    cpu_instance.step();
    Err(EmulationError::CPUStepError)
}

#[wasm_bindgen]
pub fn emulatue(data: &[u8]) {
    let mut cpu_instance = cpu::CPU::new();


    let mut do_steps = || -> Result<(), EmulationError> {
        load_rom(&mut cpu_instance, data)?;
        step_cpu(&mut cpu_instance)?;
        Ok(())
    };

    if let Err(_err) = do_steps() {
        println!("Failed to perform necessary steps");
    }

    println!("{:?}", cpu_instance.registers.a);
}



// i/o
// - Gamepad API and custom Keybaord API in js
//   Store input state in Memory Buffer for lookup from webassambly
// - OffscreenCanvas no work in safari so.... 
//   and SharedArrayBuffer not supported so..... 
//   copy buffer I guess or like WebAssembly.Memory or something 




