//! Panel：HUD 面板 = ChamferedBox + Token 推导的外观（计划书 1.2 容器/布局 P0）。

use hud_core::{Appearance, ChamferedBox};
use hud_tokens::{Chamfer, Tokens};

/// 切角档位（引用 Token 档位，避免临时数值）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChamferLevel {
    /// 4px
    S,
    /// 8px
    M,
    /// 12px
    L,
}

impl ChamferLevel {
    /// 档位 → Token 数值。
    pub fn value(self, chamfer: &Chamfer) -> f32 {
        match self {
            Self::S => chamfer.s,
            Self::M => chamfer.m,
            Self::L => chamfer.l,
        }
    }
}

/// 由 Token 推导切角外观（组件内禁止字面颜色/尺寸，全部经此函数）。
pub fn appearance(tokens: &Tokens, level: ChamferLevel) -> Appearance {
    let p = &tokens.palette;
    Appearance {
        chamfer: level.value(&tokens.geometry.chamfer),
        fill: hud_theme::color(p.bg1),
        border_color: hud_theme::color(p.line_base),
        border_width: tokens.geometry.line.base,
    }
}

/// 创建 HUD 面板（切角背景 + 内容自动避让）。
pub fn panel<'a, Message, Renderer>(
    content: impl Into<iced::Element<'a, Message, HudTheme, Renderer>>,
    tokens: &Tokens,
    level: ChamferLevel,
) -> ChamferedBox<'a, Message, HudTheme, Renderer>
where
    Renderer:
        iced::advanced::graphics::geometry::Renderer + iced::advanced::text::Renderer + 'static,
{
    ChamferedBox::new(content, appearance(tokens, level))
}

use hud_theme::HudTheme;

/// 线宽档位助手（供需要 2px 强调线的组件使用）。
pub fn strong_line(tokens: &Tokens) -> f32 {
    tokens.geometry.line.strong
}
