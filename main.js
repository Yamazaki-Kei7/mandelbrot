import init, { Renderer } from './pkg/mandelbrot.js';

async function run() {
    await init();

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d');

    // Canvasサイズをウィンドウサイズに合わせる
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const renderer = new Renderer(canvas.width, canvas.height);
    const imageData = ctx.createImageData(canvas.width, canvas.height);
    const data = new Uint8ClampedArray(imageData.data.buffer);

    let needsRedraw = true;
    let isDragging = false;
    let lastX = 0;
    let lastY = 0;

    function draw() {
        if (needsRedraw) {
            renderer.render(data);
            ctx.putImageData(imageData, 0, 0);
            needsRedraw = false;
        }
        requestAnimationFrame(draw);
    }

    // ズーム: マウスホイール
    canvas.addEventListener('wheel', (e) => {
        e.preventDefault();
        const rect = canvas.getBoundingClientRect();
        const px = e.clientX - rect.left;
        const py = e.clientY - rect.top;
        const factor = e.deltaY < 0 ? 1.2 : 1 / 1.2;
        renderer.zoom(px, py, factor);
        needsRedraw = true;
    }, { passive: false });

    // パン: マウスドラッグ
    canvas.addEventListener('mousedown', (e) => {
        isDragging = true;
        lastX = e.clientX;
        lastY = e.clientY;
    });

    canvas.addEventListener('mousemove', (e) => {
        if (!isDragging) return;
        const dx = e.clientX - lastX;
        const dy = e.clientY - lastY;
        lastX = e.clientX;
        lastY = e.clientY;
        renderer.pan(dx, dy);
        needsRedraw = true;
    });

    canvas.addEventListener('mouseup', () => {
        isDragging = false;
    });

    canvas.addEventListener('mouseleave', () => {
        isDragging = false;
    });

    // 初回描画開始
    requestAnimationFrame(draw);
}

run();
