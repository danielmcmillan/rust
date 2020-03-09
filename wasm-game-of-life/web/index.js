import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 10;
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();
const canvas = document.getElementById("universe");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const playButton = document.getElementById('play-button');

const ctx = canvas.getContext('2d');
let animationId;

function draw() {
    const ptr = universe.cells();
    const cells = new Uint8Array(memory.buffer, ptr, width * height / 8);
    ctx.beginPath();
    for (let y = 0; y < height; ++y) {
        for (let x = 0; x < width; ++x) {
            const index = y * width + x;
            const cellByte = (index / 8) >>> 0;
            const byteIndex = index % 8;
            ctx.fillStyle = (cells[cellByte] & (1 << byteIndex)) === 0
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                x * (CELL_SIZE + 1) + 1,
                y * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
}

function renderLoop() {
    draw();
    universe.tick();

    animationId = requestAnimationFrame(renderLoop);
};

function play() {
    animationId = requestAnimationFrame(renderLoop);
    playButton.textContent = 'Pause';
}

function pause() {
    if (animationId) {
        cancelAnimationFrame(animationId);
        animationId = undefined;
        draw();
        playButton.textContent = 'Play';
    }
}

function handleClick(e) {
    const { left: canvasX, top: canvasY } = e.target.getBoundingClientRect();
    const { clientX, clientY } = e;
    const x = clientX - canvasX;
    const y = clientY - canvasY;
    const row = Math.floor((y - 1) / (CELL_SIZE + 1));
    const col = Math.floor((x - 1) / (CELL_SIZE + 1));
    console.log(e);
    if (e.ctrlKey) {
        universe.glider(row, col);
    } else {
        universe.toggle(row, col);
    }
    draw();
}

function handlePlayPause() {
    if (animationId) {
        pause();
    } else {
        play();
    }
}

function handleStep() {
    pause();
    universe.tick();
    draw();
}

function handleClear() {
    pause();
    universe.clear();
    draw();
}

play();
canvas.addEventListener('click', handleClick);
playButton.addEventListener('click', handlePlayPause);
document.getElementById('step-button').addEventListener('click', handleStep);
document.getElementById('clear-button').addEventListener('click', handleClear);
