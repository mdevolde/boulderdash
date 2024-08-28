import init, { Game } from './out/boulderdash.js';

async function run() {
    await init();

    let lastTime = 0;
    const tickDuration = 1000 / 5; // Par exemple, 10 ticks par seconde

    let game = await new Game();

    document.addEventListener('keydown', (event) => {
        game.key_down(event.key);
    });

    function gameLoop(timestamp) {
        const deltaTime = timestamp - lastTime;

        if (deltaTime >= tickDuration) {
            lastTime = timestamp;
            game.update();
        }

        requestAnimationFrame(gameLoop);
    }

    requestAnimationFrame(gameLoop);
}

run();