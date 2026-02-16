/// 複素平面上の表示領域の状態管理
#[derive(Debug, Clone)]
pub struct Viewport {
    /// 表示中心の実部
    pub center_re: f64,
    /// 表示中心の虚部
    pub center_im: f64,
    /// 1ピクセルあたりの複素平面上の幅
    pub scale: f64,
    /// Canvas幅（ピクセル）
    pub width: usize,
    /// Canvas高さ（ピクセル）
    pub height: usize,
}

impl Viewport {
    /// 初期ビューポート
    ///
    /// 実部 -2.0〜1.0、虚部は幅3.0をアスペクト比で調整して表示。
    pub fn new(width: usize, height: usize) -> Self {
        // 表示幅は実部 -2.0 〜 1.0 = 3.0 をベースにする
        let re_range = 3.0;
        let scale = re_range / width as f64;

        Viewport {
            center_re: -0.5,
            center_im: 0.0,
            scale,
            width,
            height,
        }
    }

    /// ピクセル座標から複素平面座標への変換
    pub fn pixel_to_complex(&self, px: usize, py: usize) -> (f64, f64) {
        let re = self.center_re + (px as f64 - self.width as f64 / 2.0) * self.scale;
        let im = self.center_im + (py as f64 - self.height as f64 / 2.0) * self.scale;
        (re, im)
    }

    /// 指定ピクセル位置を中心にズーム
    ///
    /// factor > 1.0 でズームイン、factor < 1.0 でズームアウト
    pub fn zoom(&mut self, px: f64, py: f64, factor: f64) {
        // まずピクセル位置の複素座標を取得
        let target_re = self.center_re + (px - self.width as f64 / 2.0) * self.scale;
        let target_im = self.center_im + (py - self.height as f64 / 2.0) * self.scale;

        // スケールを変更
        self.scale /= factor;

        // ズームポイントが同じピクセル位置に留まるよう中心を調整
        self.center_re = target_re - (px - self.width as f64 / 2.0) * self.scale;
        self.center_im = target_im - (py - self.height as f64 / 2.0) * self.scale;
    }

    /// 表示領域をピクセル差分だけパン
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.center_re -= dx * self.scale;
        self.center_im -= dy * self.scale;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_pixel_maps_to_center_complex() {
        let vp = Viewport::new(800, 600);
        let (re, im) = vp.pixel_to_complex(400, 300);
        assert!((re - vp.center_re).abs() < 1e-10);
        assert!((im - vp.center_im).abs() < 1e-10);
    }

    #[test]
    fn initial_viewport_covers_expected_range() {
        let vp = Viewport::new(800, 600);
        // 左上
        let (re_min, im_min) = vp.pixel_to_complex(0, 0);
        // 右下
        let (re_max, im_max) = vp.pixel_to_complex(799, 599);

        // 実部の範囲は約 -2.0 〜 1.0
        assert!(re_min < -1.9);
        assert!(re_max > 0.9);
        // 虚部は対称的
        assert!(im_min < -1.0);
        assert!(im_max > 1.0);
    }

    #[test]
    fn zoom_changes_scale() {
        let mut vp = Viewport::new(800, 600);
        let original_scale = vp.scale;
        vp.zoom(400.0, 300.0, 2.0);
        assert!((vp.scale - original_scale / 2.0).abs() < 1e-15);
    }

    #[test]
    fn zoom_at_center_preserves_center() {
        let mut vp = Viewport::new(800, 600);
        let original_center_re = vp.center_re;
        let original_center_im = vp.center_im;
        vp.zoom(400.0, 300.0, 2.0);
        assert!((vp.center_re - original_center_re).abs() < 1e-10);
        assert!((vp.center_im - original_center_im).abs() < 1e-10);
    }

    #[test]
    fn pan_moves_center() {
        let mut vp = Viewport::new(800, 600);
        let original_center_re = vp.center_re;
        let original_center_im = vp.center_im;
        let scale = vp.scale;
        vp.pan(100.0, 50.0);
        assert!((vp.center_re - (original_center_re - 100.0 * scale)).abs() < 1e-10);
        assert!((vp.center_im - (original_center_im - 50.0 * scale)).abs() < 1e-10);
    }
}
