import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

class Fps {
    constructor() {
        this.el = document.getElementById('fps');
        this.frames = [];
        this.lastFrameTimestamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.el.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
    }
}

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

const fps = new Fps();
const ctx = canvas.getContext('2d');
let paused = false;

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
    fps.render();

    draw();
    if (!paused) {
        universe.tick();
    }

    requestAnimationFrame(renderLoop);
};

function play() {
    paused = false;
    playButton.textContent = 'Pause';
}

function pause() {
    paused = true;
    playButton.textContent = 'Play';
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
    if (paused) {
        play();
    } else {
        pause();
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

requestAnimationFrame(renderLoop);
canvas.addEventListener('click', handleClick);
playButton.addEventListener('click', handlePlayPause);
document.getElementById('step-button').addEventListener('click', handleStep);
document.getElementById('clear-button').addEventListener('click', handleClear);
