//! StatusPanel：HUD 状态面板（在线 / 警告 / 离线），色块 + 主标签 + 详情小字。

use hud_theme::HudTheme;
use hud_tokens::Tokens;
use iced::widget::{column, container, row, rule, text};
use iced::{Element, Padding};

/// 运行状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpStatus {
    /// 在线（success）
    Online,
    /// 警告（accent）
    Warning,
    /// 离线（text_disabled）
    Offline,
}

/// 渲染状态面板（切角面板 + 状态色块 + 详情）。
pub fn status_panel<'a, Message: 'a>(
    label: &str,
    detail: &str,
    status: OpStatus,
    tokens: &Tokens,
    width: f32,
) -> Element<'a, Message, HudTheme> {
    let dot = container(text(""))
        .width(tokens.geometry.space.s)
        .height(tokens.geometry.space.s)
        .style(move |theme: &HudTheme| iced::widget::container::Style {
            background: Some(status_color(theme, status).into()),
            border: iced::Border {
                color: hud_theme::color(theme.tokens.palette.line_strong),
                width: theme.tokens.geometry.line.base,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    let head = row![dot, text(label.to_string()).size(14)]
        .spacing(tokens.geometry.space.s)
        .align_y(iced::alignment::Vertical::Center);

    let body = column![
        head,
        rule::horizontal(1.0),
        text(detail.to_string()).size(12).style(|theme: &HudTheme| {
            iced::widget::text::Style {
                color: Some(hud_theme::color(theme.tokens.palette.text_secondary)),
            }
        }),
    ]
    .spacing(tokens.geometry.space.s);

    crate::panel::panel(body, tokens, crate::panel::ChamferLevel::M)
        .padding(Padding::from([
            tokens.geometry.space.m,
            tokens.geometry.space.l,
        ]))
        .width(width)
        .into()
}

fn status_color(theme: &HudTheme, status: OpStatus) -> iced::Color {
    let p = &theme.tokens.palette;
    match status {
        OpStatus::Online => hud_theme::color(p.success),
        OpStatus::Warning => hud_theme::color(p.accent),
        OpStatus::Offline => hud_theme::color(p.text_disabled),
    }
}
