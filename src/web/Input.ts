
const memory = new WebAssembly.Memory({ initial: 1, maximum: 1 });

const inputStateBuffer = new Uint8Array(memory.buffer);
const statePointers: { [key: string]: number } = {};

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();

const gamepads = new Set();

let nextBufferIndex = 0;

/*
    inputStateBuffer = [
        [
            key: byte[8]  // 8 byte string for key name, eg: "Button0_" or "Button12" or "A_______" or "Enter___" or "Bckspace"
            state: byte
        ] // 9 bytes
        ...
    ];
*/

function getKeyState(keyId: string) {
    const index = getKeyStateIndex(keyId);
    const keyState = inputStateBuffer[index + 8];
    return keyState;
}

function parseInputStateToMap(inputStateBuffer: Uint8Array) {
    const map: { [key: string]: number } = {};
    let index = 0;
    let firstByte = -1;
    while (firstByte !== 0) {
        firstByte = inputStateBuffer[index];
        if (firstByte !== 0) {
            const keyBytes = inputStateBuffer.slice(index, index + 8);
            const keyState = inputStateBuffer[index + 8];
            let keyByteLen = 0;
            for (let i = 0; i < 8; i++) {
                if (keyBytes[i] === 0) break;
                keyByteLen = i;
            }
            const keyId = textDecoder.decode(keyBytes.slice(0, keyByteLen + 1));
            map[keyId] = keyState;
            index += 9;
        }
    }
    return map;
}

function pushInputState(keyId: string) {
    const byteId = textEncoder.encode(keyId);
    const stateIndex = nextBufferIndex;
    statePointers[keyId] = stateIndex;
    for (let i = 0; i < 8; i++) {
        const index = stateIndex + i;
        inputStateBuffer[index] = byteId[i];
    }
    nextBufferIndex += 9;
    return stateIndex;
}

function getKeyStateIndex(keyId: string): number {
    if (keyId in statePointers) {
        return statePointers[keyId];
    } else {
        return pushInputState(keyId);
    }
}

function updateInputState(keyId: string, state: number) {
    const stateBufferIndex = getKeyStateIndex(keyId);
    inputStateBuffer[stateBufferIndex + 8] = state;

    // TODO: remove log
    // console.log(parseInputStateToMap(inputStateBuffer));
    // console.log([getKeyState("C0Axe0"), getKeyState("C0Axe1")], [getKeyState("C0Axe2"), getKeyState("C0Axe3")]);
}

function parseKeyIndex(key: string) {
    // make sure its max 8 bytes
    switch (key) {
        case 'Backspace':   key = "Bckspace"; break;
        case 'ContextMenu': key = "ContextMenu"; break;
        case 'ArrowLeft':   key = "ArrwLeft"; break;
        case 'ArrowRight':  key = "ArrwRight"; break;
        case 'ArrowUp':     key = "ArrwUp"; break;
        case 'ArrowDown':   key = "ArrwDown"; break;
        case 'ScrollLock':  key = "ScrllLck"; break;
        case ' ':           key = "Space"; break;
    }
    return key.toLocaleLowerCase();
}

window.addEventListener('keydown', e => {
    const lastState = getKeyState(parseKeyIndex(e.key));
    if (lastState === 0) {
        updateInputState(parseKeyIndex(e.key), 1);
    }
});

window.addEventListener('keyup', e => {
    updateInputState(parseKeyIndex(e.key), 0);
});

window.addEventListener("gamepadconnected", e => {
    gamepads.add(e.gamepad);
    console.log(e.gamepad);
});

window.addEventListener("gamepaddisconnected", e => {
    gamepads.delete(e.gamepad);
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
                const keyId = `C${index}Axe${axe}`;
                const state = ((axes[axe] / 2) + 0.5) * 64;
                updateInputState(keyId, state);
            }
            for(let btn in btns) {
                const button = btns[btn];
                const keyId = `C${index}Btn${btn}`;
                const state = button.value;
                updateInputState(keyId, state);
            }
        }
    }
}

function updateGamepads() {
    pollGamepads();
    requestAnimationFrame(() => updateGamepads());
}

updateGamepads();

export default class Input {

    static test() {

    }

}
