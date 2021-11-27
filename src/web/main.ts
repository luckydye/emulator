import wasm_module from '../../lib/emulator/pkg/emulator_bg.wasm';
import { greet, default as initWasm } from '../../lib/emulator/pkg/emulator.js';

import './Input';

async function initModule() {

    await initWasm(await wasm_module());

    greet("test");
}

initModule();