use wasm_bindgen::prelude::*;

mod cpu;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn debug(string: &str) {
    log(&format!("[emulator] {}", string));
}


// memory com

const WASM_MEMORY_BUFFER_SIZE: usize = 0xFFFF;
static mut WASM_MEMORY_BUFFER: [u8; WASM_MEMORY_BUFFER_SIZE] = [0; WASM_MEMORY_BUFFER_SIZE];

#[wasm_bindgen]
pub fn get_memory_pointer() -> *const u8 {
  let pointer: *const u8;
  unsafe {
    pointer = WASM_MEMORY_BUFFER.as_ptr();
  }

  return pointer;
}


// emulator functions

fn load_rom(cpu_instance: &mut cpu::CPU, momory: &[u8]) {
    let mut mem: [u8; 0xFFFF] = [0; 0xFFFF];

    let mut i = 0;
    for _ in mem {
        if i < momory.len() {
            mem[i] = momory[i];
            i += 1;
        } else {
            break;
        }
    }

    cpu_instance.load_rom(mem);
}

#[wasm_bindgen]
pub fn emulatue(data: &[u8]) {
    let mut cpu_instance = cpu::CPU::new();
    debug("Loading rom");

    debug(&format!("{:?}", data));

    load_rom(&mut cpu_instance, &data);
    debug("Boot Rom loaded.");
    
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();
    cpu_instance.step();


    debug("Program end");

    log(&format!("\nCounters: "));
    log(&format!("sp: {:?}", cpu_instance.sp));
    log(&format!("pc: {:?}", cpu_instance.pc));

    log(&format!("\nRegisters: "));
    log(&format!("a: {:?}", cpu_instance.registers.a));
    log(&format!("b: {:?}", cpu_instance.registers.b));
    log(&format!("c: {:?}", cpu_instance.registers.c));
    log(&format!("d: {:?}", cpu_instance.registers.d));
    log(&format!("e: {:?}", cpu_instance.registers.e));
    log(&format!("f: {:?}", cpu_instance.registers.f));
    log(&format!("h: {:?}", cpu_instance.registers.h));
    log(&format!("l: {:?}", cpu_instance.registers.l));

    log(&format!("bc: {:?}", cpu_instance.registers.get_bc()));
    log(&format!("de: {:?}", cpu_instance.registers.get_de()));
    log(&format!("hl: {:?}", cpu_instance.registers.get_hl()));
}


// i/o
// - Gamepad API and custom Keybaord API in js
//   Store input state in Memory Buffer for lookup from webassambly
// - OffscreenCanvas no work in safari so.... 
//   and SharedArrayBuffer not supported so..... 
//   copy buffer I guess or like WebAssembly.Memory or something 




