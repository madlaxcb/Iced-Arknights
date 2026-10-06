//! 装饰原语：角标、刻度分隔线、网格背景、斜纹条。
//!
//! 全部经由 `canvas::Cache` 缓存绘制（ADR-002 强制），颜色与间距只读 Token。

use hud_theme::HudTheme;
use iced::widget::canvas;
use iced::{Color, Point, Size};

macro_rules! widget_common {
    ($name:ident) => {
        impl<Message> canvas::Program<Message, HudTheme> for $name {
            type State = ();

            fn draw(
                &self,
                _state: &Self::State,
                renderer: &iced::Renderer,
                theme: &HudTheme,
                bounds: iced::Rectangle,
                _cursor: iced::mouse::Cursor,
            ) -> Vec<canvas::Geometry> {
                let frame = self.cache.draw(renderer, bounds.size(), |frame| {
                    self.draw_content(frame, bounds.size(), theme);
                });
                vec![frame]
            }
        }
    };
}

/// 面板四角的 L 形角标（可开关的装饰，计划书 1.2 特色 / 装饰）。
pub struct CornerBrackets {
    /// 角标臂长（Token 间距口径）
    pub length: f32,
    /// 线宽（1px 常规 / 2px 强调）
    pub thickness: f32,
    /// 使用强调色（默认弱色）
    pub accent: bool,
    cache: canvas::Cache,
}

impl CornerBrackets {
    /// 创建角标（默认 16px 臂长、1px 线、弱色）。
    pub fn new() -> Self {
        Self {
            length: 16.0,
            thickness: 1.0,
            accent: false,
            cache: canvas::Cache::new(),
        }
    }

    /// 使用强调色。
    pub fn accent(mut self) -> Self {
        self.accent = true;
        self
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;
        let color = if self.accent { p.accent } else { p.line_strong };
        let l = self.length;
        let w = self.thickness;
        let stroke = canvas::Stroke::default()
            .with_color(hud_theme::color(color))
            .with_width(w);

        let corners = [
            // (角点, x 方向, y 方向)
            (Point::new(0.0, 0.0), 1.0, 1.0),
            (Point::new(size.width, 0.0), -1.0, 1.0),
            (Point::new(size.width, size.height), -1.0, -1.0),
            (Point::new(0.0, size.height), 1.0, -1.0),
        ];
        for (corner, dx, dy) in corners {
            let path = canvas::Path::new(|path| {
                path.move_to(Point::new(corner.x + dx * l, corner.y));
                path.line_to(corner);
                path.line_to(Point::new(corner.x, corner.y + dy * l));
            });
            frame.stroke(&path, stroke);
        }
    }
}

impl Default for CornerBrackets {
    fn default() -> Self {
        Self::new()
    }
}

widget_common!(CornerBrackets);

/// 刻度分隔线：基线 + 均匀刻度。
pub struct TickDivider {
    /// 刻度间距（Token 间距口径）
    pub spacing: f32,
    /// 刻度高度
    pub tick_height: f32,
    cache: canvas::Cache,
}

impl TickDivider {
    /// 创建刻度分隔线（默认 8px 间距、4px 刻度）。
    pub fn new() -> Self {
        Self {
            spacing: 8.0,
            tick_height: 4.0,
            cache: canvas::Cache::new(),
        }
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;
        let mid = size.height / 2.0;
        let line = canvas::Path::new(|path| {
            path.move_to(Point::new(0.0, mid));
            path.line_to(Point::new(size.width, mid));
        });
        frame.stroke(
            &line,
            canvas::Stroke::default()
                .with_color(hud_theme::color(p.line_base))
                .with_width(theme.tokens.geometry.line.base),
        );

        let mut x = 0.0;
        while x <= size.width {
            let tick = canvas::Path::new(|path| {
                path.move_to(Point::new(x, mid - self.tick_height / 2.0));
                path.line_to(Point::new(x, mid + self.tick_height / 2.0));
            });
            frame.stroke(
                &tick,
                canvas::Stroke::default()
                    .with_color(hud_theme::color(p.line_dim))
                    .with_width(theme.tokens.geometry.line.base),
            );
            x += self.spacing.max(2.0);
        }
    }
}

impl Default for TickDivider {
    fn default() -> Self {
        Self::new()
    }
}

widget_common!(TickDivider);

/// 低对比度网格背景（缓存绘制，计划书 M3）。
pub struct GridBackground {
    /// 网格间距（Token 间距口径）
    pub spacing: f32,
    /// 透明度（0.0-1.0，默认低对比度）
    pub alpha: f32,
    cache: canvas::Cache,
}

impl GridBackground {
    /// 创建网格背景（默认 12px 间距、0.35 透明度）。
    pub fn new() -> Self {
        Self {
            spacing: 12.0,
            alpha: 0.35,
            cache: canvas::Cache::new(),
        }
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;
        let line_color: Color = {
            let c = hud_theme::color(p.line_dim);
            Color {
                a: c.a * self.alpha,
                ..c
            }
        };
        let stroke = canvas::Stroke::default()
            .with_color(line_color)
            .with_width(theme.tokens.geometry.line.base);

        let mut x = 0.0;
        while x <= size.width {
            let path = canvas::Path::new(|path| {
                path.move_to(Point::new(x, 0.0));
                path.line_to(Point::new(x, size.height));
            });
            frame.stroke(&path, stroke);
            x += self.spacing.max(2.0);
        }
        let mut y = 0.0;
        while y <= size.height {
            let path = canvas::Path::new(|path| {
                path.move_to(Point::new(0.0, y));
                path.line_to(Point::new(size.width, y));
            });
            frame.stroke(&path, stroke);
            y += self.spacing.max(2.0);
        }
    }
}

impl Default for GridBackground {
    fn default() -> Self {
        Self::new()
    }
}

widget_common!(GridBackground);

/// 45° 斜纹条（装饰条带）。
pub struct SlantedStripes {
    /// 斜线间距
    pub spacing: f32,
    /// 线宽
    pub thickness: f32,
    /// 使用强调色（默认弱色）
    pub accent: bool,
    cache: canvas::Cache,
}

impl SlantedStripes {
    /// 创建斜纹条（默认 6px 间距、1px 线、弱色）。
    pub fn new() -> Self {
        Self {
            spacing: 6.0,
            thickness: 1.0,
            accent: false,
            cache: canvas::Cache::new(),
        }
    }

    /// 使用强调色。
    pub fn accent(mut self) -> Self {
        self.accent = true;
        self
    }

    fn draw_content(&self, frame: &mut canvas::Frame, size: Size, theme: &HudTheme) {
        let p = &theme.tokens.palette;
        let color = if self.accent { p.accent } else { p.line_dim };
        let stroke = canvas::Stroke::default()
            .with_color(hud_theme::color(color))
            .with_width(self.thickness);

        // 45° 平行斜线：从顶边 (x,0) 到右边/底边 (x+h, h)
        let h = size.height;
        let step = self.spacing.max(2.0);
        let mut x = -h;
        while x <= size.width {
            let path = canvas::Path::new(|path| {
                path.move_to(Point::new(x, 0.0));
                path.line_to(Point::new(x + h, h));
            });
            frame.stroke(&path, stroke);
            x += step;
        }
    }
}

impl Default for SlantedStripes {
    fn default() -> Self {
        Self::new()
    }
}

widget_common!(SlantedStripes);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoration_params_are_on_positive_scale() {
        let b = CornerBrackets::new();
        assert!(b.length > 0.0 && b.thickness > 0.0);
        let d = TickDivider::default();
        assert!(d.spacing >= 2.0);
        let g = GridBackground::default();
        assert!((0.0..=1.0).contains(&g.alpha));
        let s = SlantedStripes::default();
        assert!(s.spacing >= 2.0);
    }
}
