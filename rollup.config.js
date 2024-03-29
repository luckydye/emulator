import typescript from '@rollup/plugin-typescript';
import { wasm } from '@rollup/plugin-wasm';
import node_resolve from '@rollup/plugin-node-resolve';

const production = false;

export default [
    {
        input: 'src/web/main.ts',
        output: {
            sourcemap: !production,
            file: 'build/main.js',
        },
        plugins: [
            node_resolve(),
            typescript(),
            wasm(),
        ]
    }
];
