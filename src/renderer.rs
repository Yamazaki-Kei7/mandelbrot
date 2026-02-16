use wasm_bindgen::prelude::*;

use crate::color::apply_colormap;
use crate::core::compute_region;
use crate::viewport::Viewport;

/// マンデルブロ集合のWasmレンダラー
///
/// JS側から渡されるImageDataバッファに直接描画結果を書き込む。
#[wasm_bindgen]
pub struct Renderer {
    viewport: Viewport,
    iteration_buffer: Vec<u32>,
    smooth_buffer: Vec<f64>,
    max_iterations: u32,
}

#[wasm_bindgen]
impl Renderer {
    /// 新しいRendererを生成する
    ///
    /// width, height はCanvasのピクセルサイズ。
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Renderer {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let pixel_count = width * height;
        Renderer {
            viewport: Viewport::new(width, height),
            iteration_buffer: vec![0u32; pixel_count],
            smooth_buffer: vec![0.0f64; pixel_count],
            max_iterations: 256,
        }
    }

    /// JS提供のImageDataバッファに描画結果を書き込む
    ///
    /// data は Uint8ClampedArray (length = width * height * 4)
    pub fn render(&mut self, data: &mut [u8]) {
        let width = self.viewport.width;
        let height = self.viewport.height;

        compute_region(
            &mut self.iteration_buffer,
            &mut self.smooth_buffer,
            width,
            height,
            &self.viewport,
            self.max_iterations,
        );

        apply_colormap(
            &self.iteration_buffer,
            &self.smooth_buffer,
            data,
            self.max_iterations,
        );
    }

    /// ビューポートのズーム操作
    pub fn zoom(&mut self, px: f64, py: f64, factor: f64) {
        self.viewport.zoom(px, py, factor);
    }

    /// ビューポートのパン操作
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.viewport.pan(dx, dy);
    }

    /// 最大反復回数の設定
    pub fn set_max_iterations(&mut self, max: u32) {
        self.max_iterations = max;
    }

    /// Canvas幅を返す
    pub fn width(&self) -> usize {
        self.viewport.width
    }

    /// Canvas高さを返す
    pub fn height(&self) -> usize {
        self.viewport.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renderer_creates_with_correct_size() {
        let renderer = Renderer::new(100, 80);
        assert_eq!(renderer.width(), 100);
        assert_eq!(renderer.height(), 80);
    }

    #[test]
    fn render_fills_buffer_with_valid_rgba() {
        let mut renderer = Renderer::new(10, 10);
        let mut data = vec![0u8; 10 * 10 * 4];
        renderer.render(&mut data);

        // すべてのアルファ値が255であること
        for i in 0..100 {
            assert_eq!(data[i * 4 + 3], 255, "pixel {i} alpha should be 255");
        }
    }

    #[test]
    fn render_produces_both_black_and_colored_pixels() {
        let mut renderer = Renderer::new(100, 100);
        let mut data = vec![0u8; 100 * 100 * 4];
        renderer.render(&mut data);

        let mut has_black = false;
        let mut has_colored = false;
        for i in 0..10000 {
            let r = data[i * 4];
            let g = data[i * 4 + 1];
            let b = data[i * 4 + 2];
            if r == 0 && g == 0 && b == 0 {
                has_black = true;
            } else {
                has_colored = true;
            }
        }
        assert!(has_black, "should have black pixels (inside set)");
        assert!(has_colored, "should have colored pixels (outside set)");
    }

    #[test]
    fn zoom_changes_render_output() {
        let mut renderer = Renderer::new(10, 10);
        let mut data1 = vec![0u8; 10 * 10 * 4];
        renderer.render(&mut data1);

        renderer.zoom(5.0, 5.0, 2.0);
        let mut data2 = vec![0u8; 10 * 10 * 4];
        renderer.render(&mut data2);

        assert_ne!(data1, data2, "zoomed render should differ");
    }

    #[test]
    fn pan_changes_render_output() {
        let mut renderer = Renderer::new(10, 10);
        let mut data1 = vec![0u8; 10 * 10 * 4];
        renderer.render(&mut data1);

        renderer.pan(50.0, 0.0);
        let mut data2 = vec![0u8; 10 * 10 * 4];
        renderer.render(&mut data2);

        assert_ne!(data1, data2, "panned render should differ");
    }
}
