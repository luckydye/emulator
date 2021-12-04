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

async function initModule() {
    await emulator.default(await emulatorModule());

    log('wasm module loaded');

    Input.onInput((obj: any) => {
        log(obj);

        emulator.set_input_state(Input.getMemory());
    });

    const rom = await (await fetch('../roms/Pokemon-Red-(UE).gb')).arrayBuffer();

    emulator.emulatue(new Uint8Array(rom));
}

initModule();