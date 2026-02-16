/// マンデルブロ集合の反復計算結果
#[derive(Debug, Clone, Copy)]
pub struct IterationResult {
    pub iterations: u32,
    /// スムーズカラーリング用の連続値（発散時のみ有効）
    pub smooth_value: f64,
    pub diverged: bool,
}

/// 単一ピクセルの反復計算を実行する
///
/// z = z² + c の反復を行い、|z| > 2.0 で発散と判定する。
/// スムーズカラーリング用の連続値も計算する。
pub fn compute_pixel(cr: f64, ci: f64, max_iterations: u32) -> IterationResult {
    let mut zr = 0.0_f64;
    let mut zi = 0.0_f64;
    let mut iterations = 0u32;

    while iterations < max_iterations {
        let zr2 = zr * zr;
        let zi2 = zi * zi;

        if zr2 + zi2 > 4.0 {
            // スムーズカラーリング用の連続値を計算
            // log2(log2(|z|²)) を使って滑らかな反復回数を算出
            let log_zn = (zr2 + zi2).ln() / 2.0;
            let smooth_value = iterations as f64 + 1.0 - log_zn.ln() / std::f64::consts::LN_2;
            return IterationResult {
                iterations,
                smooth_value,
                diverged: true,
            };
        }

        zi = 2.0 * zr * zi + ci;
        zr = zr2 - zi2 + cr;
        iterations += 1;
    }

    IterationResult {
        iterations,
        smooth_value: iterations as f64,
        diverged: false,
    }
}

/// ビューポート全体の一括計算
///
/// 各ピクセルの反復回数とスムーズ値を別々のバッファに書き込む。
pub fn compute_region(
    iteration_buffer: &mut [u32],
    smooth_buffer: &mut [f64],
    width: usize,
    height: usize,
    viewport: &crate::viewport::Viewport,
    max_iterations: u32,
) {
    debug_assert_eq!(iteration_buffer.len(), width * height);
    debug_assert_eq!(smooth_buffer.len(), width * height);

    for y in 0..height {
        for x in 0..width {
            let (cr, ci) = viewport.pixel_to_complex(x, y);
            let result = compute_pixel(cr, ci, max_iterations);
            let idx = y * width + x;
            iteration_buffer[idx] = result.iterations;
            smooth_buffer[idx] = result.smooth_value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_does_not_diverge() {
        // (0, 0) はマンデルブロ集合の内部
        let result = compute_pixel(0.0, 0.0, 1000);
        assert!(!result.diverged);
        assert_eq!(result.iterations, 1000);
    }

    #[test]
    fn far_point_diverges_immediately() {
        // (2, 2) は集合外で即座に発散
        let result = compute_pixel(2.0, 2.0, 1000);
        assert!(result.diverged);
        assert!(result.iterations <= 2);
    }

    #[test]
    fn minus_one_does_not_diverge() {
        // (-1, 0) は集合内（周期2の固定点）
        let result = compute_pixel(-1.0, 0.0, 1000);
        assert!(!result.diverged);
        assert_eq!(result.iterations, 1000);
    }

    #[test]
    fn boundary_point_has_expected_iterations() {
        // (-0.75, 0.1) は集合境界付近で多めの反復回数
        let result = compute_pixel(-0.75, 0.1, 1000);
        assert!(result.diverged);
        assert!(result.iterations > 10);
    }

    #[test]
    fn smooth_value_is_continuous() {
        // 発散するピクセルのsmooth_valueが反復回数付近の値
        let result = compute_pixel(0.5, 0.5, 100);
        assert!(result.diverged);
        assert!(result.smooth_value > 0.0);
        assert!(result.smooth_value < 100.0);
    }

    #[test]
    fn compute_region_matches_individual() {
        let viewport = crate::viewport::Viewport::new(4, 4);
        let mut iter_buf = vec![0u32; 16];
        let mut smooth_buf = vec![0.0f64; 16];
        compute_region(&mut iter_buf, &mut smooth_buf, 4, 4, &viewport, 100);

        for y in 0..4 {
            for x in 0..4 {
                let (cr, ci) = viewport.pixel_to_complex(x, y);
                let result = compute_pixel(cr, ci, 100);
                let idx = y * 4 + x;
                assert_eq!(iter_buf[idx], result.iterations);
                assert!((smooth_buf[idx] - result.smooth_value).abs() < 1e-10);
            }
        }
    }
}
