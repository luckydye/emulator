// @ts-ignore ignore this mod not found error cause its just how it is
import emulatorModule from '../../lib/emulator/pkg/emulator_bg.wasm';
import * as emulator from '../../lib/emulator/pkg/emulator.js';

import Input from './Input';

declare global {
    function log(str: string): void;
}

window.log = (str: string) => {
    console.log(str);
}

async function sleep(s: number): Promise<boolean> {
    return new Promise((resolve) => {
        setTimeout(() => resolve(true), s * 1000);
    })
}

async function initModule() {
    const thread = await emulator.default(await emulatorModule());
    
    const wasmMemory = new Uint8Array(thread.memory.buffer);
    let bufferPointer = thread.get_memory_pointer();
    
    wasmMemory[bufferPointer + 1] = 15;


    await sleep(1);

    log('wasm module loaded');

    Input.onInput((obj: any) => {
        log(obj);

        emulator.set_input_state(Input.getMemory());
    });

    const rom = await (await fetch('../roms/boot-rom.gb')).arrayBuffer();

    console.log(rom);

    emulator.emulatue(new Uint8Array(rom));
}

initModule();