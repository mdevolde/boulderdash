import init, { GameManager } from './out/boulderdash.js';

async function run() {
    await init();

    let lastTime = 0;
    const tickDuration = 1000 / 20;
    let game = new GameManager();
    let gameStarted = false;
    const keysPressed = {};

    let startX, startY, endX, endY;

    document.addEventListener('touchstart', function(event) {
        const touch = event.touches[0];
        startX = touch.clientX;
        startY = touch.clientY;
    }, false);

    document.addEventListener('touchend', function(event) {
        handleSwipeRelease();
    }, false);

    function handleSwipeRelease() {
        keysPressed['ArrowRight'] = false;
        keysPressed['ArrowLeft'] = false;
        keysPressed['ArrowUp'] = false;
        keysPressed['ArrowDown'] = false;
    }

    document.addEventListener('touchmove', function(event) {
        const touch = event.touches[0];
        endX = touch.clientX;
        endY = touch.clientY;
        handleSwipe();
    }, false);

    function handleSwipe() {
        const deltaX = endX - startX;
        const deltaY = endY - startY;
        
        keysPressed['ArrowRight'] = false;
        keysPressed['ArrowLeft'] = false;
        keysPressed['ArrowUp'] = false;
        keysPressed['ArrowDown'] = false;

        if (Math.abs(deltaX) > Math.abs(deltaY)) {
            if (deltaX > 0) {
                keysPressed['ArrowRight'] = true;
            } else {
                keysPressed['ArrowLeft'] = true;
            }
        } else {
            if (deltaY > 0) {
                keysPressed['ArrowDown'] = true;
            } else {
                keysPressed['ArrowUp'] = true;
            }
        }
    }

    document.addEventListener('keydown', async (event) => {
        if (!gameStarted) {
            gameStarted = true;
            await game.start();
            requestAnimationFrame(gameLoop);
        } else {
            keysPressed[event.key] = true;
        }
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
}

run();
