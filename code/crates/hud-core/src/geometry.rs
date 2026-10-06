//! 切角矩形几何与像素对齐（纯函数，独立于渲染器，可完全单测）。

/// 将切角尺寸限制在矩形短边的一半以内。
pub fn chamfer_clamp(width: f32, height: f32, chamfer: f32) -> f32 {
    if width <= 0.0 || height <= 0.0 {
        return 0.0;
    }
    chamfer.clamp(0.0, width.min(height) / 2.0)
}

/// 计算 45° 切角矩形的 8 个顶点（顺时针，从顶边左端开始）。
///
/// 返回值为 `(x, y)` 局部坐标（原点在矩形左上角）。
pub fn chamfered_rect_points(width: f32, height: f32, chamfer: f32) -> [(f32, f32); 8] {
    let c = chamfer_clamp(width, height, chamfer);
    let r = width;
    let b = height;
    [
        (c, 0.0),
        (r - c, 0.0),
        (r, c),
        (r, b - c),
        (r - c, b),
        (c, b),
        (0.0, b - c),
        (0.0, c),
    ]
}

/// 将逻辑坐标对齐到物理像素网格（缓解 125% / 150% 缩放下 1px 线条发虚）。
///
/// `scale` 为 DPI 缩放系数（如 1.0 / 1.25 / 1.5 / 2.0）。
pub fn snap_to_pixel(logical: f32, scale: f32) -> f32 {
    if scale <= 0.0 {
        return logical;
    }
    (logical * scale).round() / scale
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn points_are_clockwise_eight() {
        let pts = chamfered_rect_points(100.0, 50.0, 8.0);
        assert_eq!(pts.len(), 8);
        assert!(close(pts[0].0, 8.0) && close(pts[0].1, 0.0));
        assert!(close(pts[2].0, 100.0) && close(pts[2].1, 8.0));
        assert!(close(pts[4].0, 92.0) && close(pts[4].1, 50.0));
        assert!(close(pts[6].0, 0.0) && close(pts[6].1, 42.0));
    }

    #[test]
    fn chamfer_clamped_to_half_short_side() {
        assert!(close(chamfer_clamp(20.0, 10.0, 8.0), 5.0));
        assert!(close(chamfer_clamp(20.0, 10.0, 4.0), 4.0));
        assert_eq!(chamfer_clamp(0.0, 10.0, 4.0), 0.0);
        assert!(close(chamfer_clamp(10.0, 10.0, 100.0), 5.0));
    }

    #[test]
    fn zero_chamfer_yields_rect_corners() {
        let pts = chamfered_rect_points(100.0, 50.0, 0.0);
        assert!(close(pts[0].0, 0.0) && close(pts[1].0, 100.0));
        assert!(close(pts[4].0, 100.0) && close(pts[4].1, 50.0));
    }

    #[test]
    fn snap_keeps_values_stable_at_1x() {
        assert!(close(snap_to_pixel(10.4, 1.0), 10.0));
        assert!(close(snap_to_pixel(10.6, 1.0), 11.0));
    }

    #[test]
    fn snap_lands_on_physical_pixels_at_fractional_scales() {
        for scale in [1.25_f32, 1.5, 2.0] {
            let snapped = snap_to_pixel(10.3, scale);
            let physical = snapped * scale;
            assert!(
                close(physical, physical.round()),
                "scale {scale}: {physical} 不在物理像素上"
            );
        }
    }

    #[test]
    fn snap_is_idempotent() {
        let once = snap_to_pixel(10.3, 1.25);
        let twice = snap_to_pixel(once, 1.25);
        assert!(close(once, twice));
    }
}
