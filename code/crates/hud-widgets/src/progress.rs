//! Progress 进度条：线性（主题化内置）/ 分段 / 斜切填充（canvas 缓存绘制）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::canvas;
use iced::{Point, Rectangle, Size};

/// 分段进度条：`segments` 段中填充 `value * segments` 段。
pub struct SegmentedProgress {
    /// 进度 0.0 ~ 1.0
    pub value: f32,
    /// 段数
    pub segments: usize,
    cache: canvas::Cache,
}

impl SegmentedProgress {
    /// 创建分段进度条。
    pub fn new(value: f32, segments: usize) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            segments: segments.max(1),
            cache: canvas::Cache::new(),
        }
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;
        let gap = 2.0;
        let seg_w = (size.width - gap * (self.segments as f32 - 1.0)) / self.segments as f32;
        let filled = (self.value * self.segments as f32).round() as usize;

        for i in 0..self.segments {
            let x = i as f32 * (seg_w + gap);
            let rect = Rectangle::new(Point::new(x, 0.0), Size::new(seg_w, size.height));
            let color = if i < filled { p.accent } else { p.bg2 };
            frame.fill_rectangle(rect.position(), rect.size(), hud_theme::color(color));
        }

        // 外边框（1px 常规线）
        let border = canvas::Path::rectangle(Point::ORIGIN, size);
        frame.stroke(
            &border,
            canvas::Stroke::default()
                .with_color(hud_theme::color(p.line_base))
                .with_width(theme.tokens.geometry.line.base),
        );
    }
}

impl<Message> canvas::Program<Message, HudTheme> for SegmentedProgress {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &HudTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let frame = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_content(frame, bounds.size(), theme);
        });
        vec![frame]
    }
}

/// 斜切进度条：进度区以 45° 斜纹填充（HUD 风格纹理）。
pub struct SlantedProgress {
    /// 进度 0.0 ~ 1.0
    pub value: f32,
    cache: canvas::Cache,
}

impl SlantedProgress {
    /// 创建斜切进度条。
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            cache: canvas::Cache::new(),
        }
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;

        // 轨道
        frame.fill_rectangle(Point::ORIGIN, size, hud_theme::color(p.bg2));

        // 进度填充
        let fill_w = size.width * self.value;
        if fill_w > 0.0 {
            frame.fill_rectangle(
                Point::ORIGIN,
                Size::new(fill_w, size.height),
                hud_theme::color(p.accent),
            );

            // 斜纹（暗色 45° 线，裁剪在填充区内）
            let step = 6.0;
            let h = size.height;
            let mut x = -h;
            while x < fill_w {
                let clip_x = x.max(0.0);
                let y0 = clip_x - x; // 折算起点 y，保持 45°
                let end_x = (x + h).min(fill_w);
                let path = canvas::Path::new(|path| {
                    path.move_to(Point::new(clip_x, y0));
                    path.line_to(Point::new(end_x, h));
                });
                frame.stroke(
                    &path,
                    canvas::Stroke::default()
                        .with_color(hud_theme::color(p.bg0))
                        .with_width(1.5),
                );
                x += step;
            }
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

impl<Message> canvas::Program<Message, HudTheme> for SlantedProgress {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        theme: &HudTheme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let frame = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_content(frame, bounds.size(), theme);
        });
        vec![frame]
    }
}

/// 由 Token 计算进度条推荐高度（间距档位口径，组件内无字面尺寸）。
pub fn recommended_height(tokens: &Tokens) -> f32 {
    tokens.geometry.space.s
}
