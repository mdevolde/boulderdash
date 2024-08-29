import init, { Game } from './out/boulderdash.js';

async function run() {
    await init();

    let lastTime = 0;
    const tickDuration = 1000 / 10;
    let game = await new Game();
    const keysPressed = {};

    document.addEventListener('keydown', (event) => {
        keysPressed[event.key] = true;
    });

    document.addEventListener('keyup', (event) => {
        keysPressed[event.key] = false;
    });

    function gameLoop(timestamp) {
        const deltaTime = timestamp - lastTime;

        if (deltaTime >= tickDuration) {
            lastTime = timestamp;

            if (keysPressed['ArrowRight']) {
                game.key_down('ArrowRight');
            }
            if (keysPressed['ArrowLeft']) {
                game.key_down('ArrowLeft');
            }
            if (keysPressed['ArrowUp']) {
                game.key_down('ArrowUp');
            }
            if (keysPressed['ArrowDown']) {
                game.key_down('ArrowDown');
            }

            game.update();
        }

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();
