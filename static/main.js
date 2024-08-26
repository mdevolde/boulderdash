import init, { draw } from './out/boulderdash.js';

async function run() {
    await init();
    draw();
}

run();