//! Loading 扫描条：全库唯一允许的循环动效（计划书 1.2 / 5.5）。
//!
//! 受控组件：`phase`（0.0-1.0，循环）由应用的时间订阅推进；**只在可见时订阅**。
//! 用法：`ScanBar::new(phase)` 每次推进后由应用重建（内部 Cache 随实例更新），
//! 配合 `iced::time::every` 等周期订阅实现扫描动画。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::canvas;
use iced::{Point, Rectangle, Size};

/// 扫描条（高亮窗口在轨道上循环移动）。
pub struct ScanBar {
    /// 相位 0.0 ~ 1.0（循环）
    pub phase: f32,
}

impl ScanBar {
    /// 创建扫描条。
    pub fn new(phase: f32) -> Self {
        Self {
            phase: phase.fract().abs(),
        }
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;

        // 轨道
        frame.fill_rectangle(Point::ORIGIN, size, hud_theme::color(p.bg2));

        // 高亮窗口（宽 1/4，三段渐隐模拟拖尾）
        let window = size.width * 0.25;
        let head = self.phase * (size.width + window) - window;
        let segments = [(0.00, 0.25), (0.25, 0.55), (0.55, 1.00)];
        for (start, end) in segments {
            let x0 = (head + window * start).max(0.0);
            let x1 = (head + window * end).min(size.width);
            if x1 <= x0 {
                continue;
            }
            let mut c = hud_theme::color(p.accent);
            c.a *= end;
            frame.fill_rectangle(Point::new(x0, 0.0), Size::new(x1 - x0, size.height), c);
        }

        // 外边框
        let border = canvas::Path::rectangle(Point::ORIGIN, size);
        frame.stroke(
            &border,
            canvas::Stroke::default()
                .with_color(hud_theme::color(p.line_base))
                .with_width(theme.tokens.geometry.line.base),
        );
    }
}

impl<Message> canvas::Program<Message, HudTheme> for ScanBar {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &HudTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // 循环动画：phase 变化即需重绘，因此实例随 phase 更新而重建（见模块文档）
        let fresh = canvas::Cache::new();
        let frame = fresh.draw(renderer, bounds.size(), |frame| {
            self.draw_content(frame, bounds.size(), theme);
        });
        vec![frame]
    }
}

/// 由 Token 推荐的扫描条高度（间距档位口径）。
pub fn recommended_height(tokens: &Tokens) -> f32 {
    tokens.geometry.space.m
}
