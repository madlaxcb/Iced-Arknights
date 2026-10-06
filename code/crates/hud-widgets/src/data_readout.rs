//! DataReadout：等宽数字读数 + 小标签（HUD 数据展示，计划书 1.2）。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{column, container, row, text};
use iced::{Element, Font, Length, Padding};

/// 渲染数据读数：顶部强调短条 + 等宽大数字 + 单位 + 标签。
///
/// 数字固定使用等宽字体（`Font::MONOSPACE`），多读数并排时数值纵向对齐。
pub fn data_readout<'a, Message: 'a>(
    label: &str,
    value: &str,
    unit: Option<&str>,
    tokens: &Tokens,
) -> Element<'a, Message, HudTheme> {
    // 顶部标识：2px 强调短条（1/4 宽）+ 弱色长线
    let top_rule = row![
        container(text(""))
            .width(24.0)
            .height(tokens.geometry.line.strong)
            .style(|theme: &HudTheme| iced::widget::container::Style {
                background: Some(hud_theme::color(theme.tokens.palette.accent).into()),
                ..Default::default()
            }),
        container(text(""))
            .width(Length::Fill)
            .height(tokens.geometry.line.strong)
            .style(|theme: &HudTheme| iced::widget::container::Style {
                background: Some(hud_theme::color(theme.tokens.palette.line_dim).into()),
                ..Default::default()
            }),
    ]
    .spacing(tokens.geometry.space.xs);

    let mut value_row = row![text(value.to_string())
        .size(tokens.typography.xl)
        .font(Font::MONOSPACE),]
    .spacing(tokens.geometry.space.xs)
    .align_y(iced::alignment::Vertical::Bottom);

    if let Some(unit) = unit {
        value_row = value_row.push(
            text(unit.to_string())
                .size(tokens.typography.xxs)
                .font(Font::MONOSPACE)
                .style(|theme: &HudTheme| iced::widget::text::Style {
                    color: Some(hud_theme::color(theme.tokens.palette.text_secondary)),
                }),
        );
    }

    column![
        top_rule,
        value_row,
        text(label.to_string())
            .size(tokens.typography.xxs)
            .style(|theme: &HudTheme| iced::widget::text::Style {
                color: Some(hud_theme::color(theme.tokens.palette.text_secondary)),
            }),
    ]
    .spacing(tokens.geometry.space.xs)
    .padding(Padding::from([tokens.geometry.space.s, 0.0]))
    .into()
}
