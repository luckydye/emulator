
const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();


const BUFFER_STATE_KEY_SIZE = 2;
const BUFFER_STATE_DEVICE_SIZE = 1;
const BUFFER_STATE_STATE_SIZE = 1;
const BUFFER_STATE_SIZE = BUFFER_STATE_KEY_SIZE + BUFFER_STATE_DEVICE_SIZE + BUFFER_STATE_STATE_SIZE;

const memory = new Uint8Array(new ArrayBuffer(200 * BUFFER_STATE_SIZE));
const inputStateBuffer = new Uint8Array(memory.buffer);

const gamepads = new Set();
const statePointers: { [key: string]: number } = {};

let nextBufferIndex = 0;

/*
    inputStateBuffer = [
        [
            key: byte[2]  // 8 byte string for key name, eg: "Button0_" or "Button12" or "A_______" or "Enter___" or "Bckspace"
            device: byte
            state: byte
        ] // 9 bytes
        ...
    ];
*/

function parseInputStateToMap(inputStateBuffer: Uint8Array): any {
    const map: { [key: string]: any } = {};
    let index = 0;
    let firstByte = -1;
    while (firstByte !== 0) {
        firstByte = inputStateBuffer[index];
        if (firstByte !== 0) {
            const keyBytes = inputStateBuffer.slice(index, index + BUFFER_STATE_KEY_SIZE);
            const keyDevice = inputStateBuffer[index + BUFFER_STATE_KEY_SIZE];
            const keyState = inputStateBuffer[index + BUFFER_STATE_KEY_SIZE + BUFFER_STATE_DEVICE_SIZE];
            const keyId = textDecoder.decode(keyBytes);
            map[keyId] = { device: keyDevice, state: keyState };
            index += BUFFER_STATE_SIZE;
        }
    }
    return map;
}

function pushInputState(keyId: string, deviceIndex: number): number {
    const byteId = textEncoder.encode(keyId);
    const stateIndex = nextBufferIndex;
    statePointers[keyId] = stateIndex;
    for (let i = 0; i < BUFFER_STATE_KEY_SIZE; i++) {
        const index = stateIndex + i;
        inputStateBuffer[index] = byteId[i];
    }
    inputStateBuffer[stateIndex + BUFFER_STATE_KEY_SIZE] = deviceIndex;
    nextBufferIndex += BUFFER_STATE_SIZE;
    return stateIndex;
}

function getButtonStateValue(deviceId: number, keyId: string): number | null {
    const index = getButtonStateIndex(deviceId, keyId);
    if(index > -1) {
        return inputStateBuffer[index + BUFFER_STATE_KEY_SIZE + BUFFER_STATE_DEVICE_SIZE];
    }
    return null;
}

function getButtonStateIndex(deviceId: number, buttonId: string): number {
    for(let btnnId in statePointers) {
        if(btnnId == buttonId) {
            const pointer = statePointers[btnnId];
            const device = inputStateBuffer[pointer + BUFFER_STATE_KEY_SIZE];
            if(device == deviceId) {
                return statePointers[buttonId];
                break;
            }
        }
    }
    return -1;
}

function updateInputState(buttonId: string, state: number, deviceIndex: number) {
    let stateBufferIndex = getButtonStateIndex(deviceIndex, buttonId);
    if(stateBufferIndex === -1) {
        stateBufferIndex = pushInputState(buttonId, deviceIndex);
    }

    inputStateBuffer[stateBufferIndex + BUFFER_STATE_KEY_SIZE + BUFFER_STATE_DEVICE_SIZE] = state;

    Input.emitInput(parseInputStateToMap(inputStateBuffer));
}

function mapInputToButton(key: string): string | null {
    // make sure its max 2 bytes
    switch (key) {
        case 'Backspace':   key = "Bckspace"; break;
        case 'ContextMenu': key = "ContextMenu"; break;
        case 'ArrowLeft':   key = "ArrwLeft"; break;
        case 'ArrowRight':  key = "ArrwRight"; break;
        case 'ArrowUp':     key = "ArrwUp"; break;
        case 'ArrowDown':   key = "ArrwDown"; break;
        case 'ScrollLock':  key = "ScrllLck"; break;
    }
    if(key.length <= 2) {
        // TODO: just proxy all key within 2 bytes through for now, no mapping to any buttons
        return key.toLocaleUpperCase();
    }
    return null;
}

window.addEventListener('keydown', e => {
    const btnId = mapInputToButton(e.key);
    if(btnId != null) {
        const lastState = getButtonStateValue(0, btnId);
        if (lastState == null || lastState === 0) {
            updateInputState(btnId, 1, 0);
            e.preventDefault();
        }
    }
});

window.addEventListener('keyup', e => {
    const btnId = mapInputToButton(e.key);
    if(btnId != null) {
        updateInputState(btnId, 0, 0);
        e.preventDefault();
    }
});

window.addEventListener("gamepadconnected", e => {
    gamepads.add(e.gamepad);
    console.log('Connected gamepad:', e.gamepad.id);
});

window.addEventListener("gamepaddisconnected", e => {
    gamepads.delete(e.gamepad);
    console.log('Disconnected gamepad:', e.gamepad.id);
});

function pollGamepads() {
    const gamepads = navigator.getGamepads();

    for(let gamepad of gamepads) {
        if(gamepad) {
            const controler = gamepad as Gamepad;
            const index = controler.index;
            const axes = controler.axes;
            const btns = controler.buttons;
    
            for(let axe in axes) {
                const state = ((axes[axe] / 2) + 0.5) * 64;
                const btnId = mapInputToButton(`A${axe}`);
                if(btnId != null) {
                    updateInputState(btnId, state, index + 1);
                }
            }
            for(let btn in btns) {
                const button = btns[btn];
                const btnId = mapInputToButton(`B${btn}`);
                if(btnId != null) {
                    updateInputState(btnId, button.value, index + 1);
                }
            }
        }
    }
}

function updateGamepads() {
    pollGamepads();
    requestAnimationFrame(() => updateGamepads());
}

updateGamepads();

const listeners: Set<Function> = new Set();

export default class Input {

    static getMemory(): Uint8Array {
        return inputStateBuffer;
    }

    static onInput(callback: Function) {
        listeners.add(callback);
        return () => {
            listeners.delete(callback);
        }
    }

    static emitInput(data: any) {
        for(let listner of listeners) {
            listner(data);
        }
    }

}
