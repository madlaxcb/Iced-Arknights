//! Badge / Tag：色条 + 文本的小标签（含稀有度风格色条，计划书 1.2）。

use hud_theme::HudTheme;
use iced::widget::{container, row, text};
use iced::{Element, Length, Padding};

/// 色条类型（映射 Token 语义色；"稀有度"以色相+明度区分，不使用游戏素材）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeKind {
    /// 中性（line_strong）
    Neutral,
    /// 信息（info）
    Info,
    /// 成功（success）
    Success,
    /// 警示（accent）
    Warning,
    /// 危险（danger）
    Danger,
}

/// 渲染徽标：左侧 3px 色条 + 右侧文字（bg2 底 + 1px 边框）。
pub fn badge<'a, Message: 'a>(label: &str, kind: BadgeKind) -> Element<'a, Message, HudTheme> {
    let strip =
        container(text(""))
            .width(3.0)
            .height(Length::Fill)
            .style(move |theme: &HudTheme| iced::widget::container::Style {
                background: Some(kind_color(theme, kind).into()),
                ..Default::default()
            });

    let label_part = container(text(label.to_string()).size(12))
        .padding(Padding::from([2, 10]).left(8))
        .style(|theme: &HudTheme| iced::widget::container::Style {
            text_color: Some(hud_theme::color(theme.tokens.palette.text_primary)),
            ..Default::default()
        });

    let tag = row![strip, label_part].height(20);

    container(tag)
        .style(|theme: &HudTheme| iced::widget::container::Style {
            background: Some(hud_theme::color(theme.tokens.palette.bg2).into()),
            border: iced::Border {
                color: hud_theme::color(theme.tokens.palette.line_base),
                width: theme.tokens.geometry.line.base,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}

fn kind_color(theme: &HudTheme, kind: BadgeKind) -> iced::Color {
    let p = &theme.tokens.palette;
    match kind {
        BadgeKind::Neutral => hud_theme::color(p.line_strong),
        BadgeKind::Info => hud_theme::color(p.info),
        BadgeKind::Success => hud_theme::color(p.success),
        BadgeKind::Warning => hud_theme::color(p.accent),
        BadgeKind::Danger => hud_theme::color(p.danger),
    }
}
