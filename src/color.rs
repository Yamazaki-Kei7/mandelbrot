/// RGBA色値
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// HSLからRGBへの変換
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

    let (r1, g1, b1) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let m = l - c / 2.0;
    (
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}

/// 反復結果をRGBA色に変換する
///
/// 集合内（未発散）のピクセルは黒色を返す。
/// 発散したピクセルはスムーズ値に基づくHSLグラデーションで色付けする。
pub fn map_color(_iterations: u32, smooth_value: f64, diverged: bool, _max_iterations: u32) -> Rgba {
    if !diverged {
        return Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
    }

    // HSLベースのグラデーション
    // hue: smooth_valueに基づく色相（0-360度を循環）
    let hue = (smooth_value * 10.0) % 360.0;
    let saturation = 0.8;
    let lightness = 0.5;

    let (r, g, b) = hsl_to_rgb(hue, saturation, lightness);
    Rgba { r, g, b, a: 255 }
}

/// バッファ全体の一括カラーマッピング
///
/// iteration_buffer と smooth_buffer から rgba_output に直接書き込む。
pub fn apply_colormap(
    iteration_buffer: &[u32],
    smooth_buffer: &[f64],
    rgba_output: &mut [u8],
    max_iterations: u32,
) {
    debug_assert_eq!(rgba_output.len(), iteration_buffer.len() * 4);
    debug_assert_eq!(iteration_buffer.len(), smooth_buffer.len());

    for i in 0..iteration_buffer.len() {
        let diverged = iteration_buffer[i] < max_iterations;
        let color = map_color(
            iteration_buffer[i],
            smooth_buffer[i],
            diverged,
            max_iterations,
        );
        let offset = i * 4;
        rgba_output[offset] = color.r;
        rgba_output[offset + 1] = color.g;
        rgba_output[offset + 2] = color.b;
        rgba_output[offset + 3] = color.a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_diverged_pixel_is_black() {
        let color = map_color(1000, 1000.0, false, 1000);
        assert_eq!(color, Rgba { r: 0, g: 0, b: 0, a: 255 });
    }

    #[test]
    fn diverged_pixel_is_not_black() {
        let color = map_color(10, 10.5, true, 1000);
        assert_ne!(color, Rgba { r: 0, g: 0, b: 0, a: 255 });
        assert_eq!(color.a, 255);
    }

    #[test]
    fn different_smooth_values_give_different_colors() {
        let color1 = map_color(10, 10.0, true, 1000);
        let color2 = map_color(10, 15.0, true, 1000);
        // 異なるsmooth_valueで異なる色が生成される
        assert_ne!(color1, color2);
    }

    #[test]
    fn apply_colormap_writes_correct_rgba() {
        let iterations = [100u32, 0];
        let smooth = [5.5f64, 0.0];
        let mut output = vec![0u8; 8];
        apply_colormap(&iterations, &smooth, &mut output, 1000);

        // 最初のピクセル: 発散（iterations < max_iterations）→非黒色
        assert_eq!(output[3], 255); // alpha
        // 2番目のピクセル: 0反復 → 発散 → 非黒色
        assert_eq!(output[7], 255); // alpha
    }

    #[test]
    fn hsl_to_rgb_red() {
        let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn hsl_to_rgb_green() {
        let (r, g, b) = hsl_to_rgb(120.0, 1.0, 0.5);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);
    }

    #[test]
    fn hsl_to_rgb_blue() {
        let (r, g, b) = hsl_to_rgb(240.0, 1.0, 0.5);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }
}
