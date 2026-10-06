//! Button 变体类（主 / 次 / 幽灵 / 危险），全部只读 Token。

use hud_theme::HudTheme;
use iced::widget::button;
use iced::Border;

/// 按钮变体。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// 主操作：强调色填充（克制作强调，占屏 ≤10%）
    Primary,
    /// 次要：描边常规
    Secondary,
    /// 幽灵：无底无框，悬停点亮
    Ghost,
    /// 危险：danger 色
    Danger,
}

/// 变体样式实现（纯函数）。
pub fn style(theme: &HudTheme, status: button::Status, variant: ButtonVariant) -> button::Style {
    let p = &theme.tokens.palette;
    let line = theme.tokens.geometry.line.base;
    let (bg, border_color, border_width, fg) = match (variant, status) {
        // 主操作：强调色底
        (ButtonVariant::Primary, button::Status::Active) => (p.accent, p.line_strong, line, p.bg0),
        (ButtonVariant::Primary, button::Status::Hovered) => (p.accent, p.line_strong, line, p.bg0),
        (ButtonVariant::Primary, button::Status::Pressed) => (p.bg2, p.line_strong, line, p.accent),
        (ButtonVariant::Primary, button::Status::Disabled) => {
            (p.bg2, p.line_dim, line, p.text_disabled)
        }
        // 危险
        (ButtonVariant::Danger, button::Status::Hovered) => (p.danger, p.danger, line, p.bg0),
        (ButtonVariant::Danger, button::Status::Pressed) => (p.bg0, p.danger, line, p.danger),
        (ButtonVariant::Danger, status_) => {
            let disabled = matches!(status_, button::Status::Disabled);
            (
                p.bg1,
                if disabled { p.line_dim } else { p.danger },
                line,
                if disabled { p.text_disabled } else { p.danger },
            )
        }
        // 幽灵：无底无框，悬停点亮
        (ButtonVariant::Ghost, button::Status::Hovered) => (p.bg2, p.line_strong, line, p.accent),
        (ButtonVariant::Ghost, button::Status::Pressed) => (p.bg1, p.line_strong, line, p.accent),
        (ButtonVariant::Ghost, button::Status::Active) => (p.bg0, p.bg0, 0.0, p.text_secondary),
        (ButtonVariant::Ghost, button::Status::Disabled) => (p.bg0, p.bg0, 0.0, p.text_disabled),
        // 次要
        (ButtonVariant::Secondary, button::Status::Hovered) => {
            (p.bg2, p.line_strong, line, p.text_primary)
        }
        (ButtonVariant::Secondary, button::Status::Pressed) => {
            (p.bg0, p.line_strong, line, p.text_primary)
        }
        (ButtonVariant::Secondary, button::Status::Disabled) => {
            (p.bg1, p.line_dim, line, p.text_disabled)
        }
        (ButtonVariant::Secondary, button::Status::Active) => {
            (p.bg1, p.line_base, line, p.text_primary)
        }
    };

    button::Style {
        background: Some(hud_theme::color(bg).into()),
        text_color: hud_theme::color(fg),
        border: Border {
            color: hud_theme::color(border_color),
            width: border_width,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

/// 变体类（装箱后的 StyleFn，可直接传给 `.class()`）。
pub fn class(variant: ButtonVariant) -> button::StyleFn<'static, HudTheme> {
    Box::new(move |theme, status| style(theme, status, variant))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_all_statuses_total() {
        let theme = HudTheme::dark();
        for variant in [
            ButtonVariant::Primary,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Danger,
        ] {
            for status in [
                button::Status::Active,
                button::Status::Hovered,
                button::Status::Pressed,
                button::Status::Disabled,
            ] {
                let _ = style(&theme, status, variant);
            }
        }
    }
}
